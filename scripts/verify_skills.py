from __future__ import annotations

import sys
from pathlib import Path
from typing import Any

import yaml


METHOD_IDS = (
    "risk-audit",
    "diff",
    "threat-model",
    "architecture",
    "attack-path",
    "red-team",
    "ctf",
    "variant-analysis",
    "invariant",
    "fuzzing",
    "formal",
    "supply-chain",
)
LINK_TYPES = {"complements", "precedes", "validates", "escalates_to", "alternative_to"}
SYMMETRIC_LINK_TYPES = {"complements", "alternative_to"}
METHOD_FILES = {
    "SKILL.md",
    "agents/openai.yaml",
    "references/l2-playbook.md",
    "references/l3-lineage.md",
}
ROUTER_FILES = {
    "SKILL.md",
    "agents/openai.yaml",
    "references/method-index.yaml",
    "references/review-contract.md",
}
L2_HEADINGS = {
    "## Tools",
    "## Practices",
    "## Protocol",
    "## Evidence and falsification",
    "## Dynamic boundary and failure behavior",
}
L3_HEADINGS = {
    "## Pressure",
    "## Method response",
    "## Inherited strengths",
    "## Known failure modes",
    "## Current shape",
    "## Primary anchors",
}
COST_ORDER = {"low": 0, "medium": 1, "high": 2}
EXPECTED_LINKS = 61


def _yaml(path: Path, errors: list[str]) -> dict[str, Any]:
    try:
        value = yaml.safe_load(path.read_text())
    except (OSError, yaml.YAMLError) as exc:
        errors.append(f"cannot parse {path}: {exc}")
        return {}
    if not isinstance(value, dict):
        errors.append(f"expected YAML mapping in {path}")
        return {}
    return value


def _frontmatter(path: Path, errors: list[str]) -> dict[str, Any]:
    try:
        text = path.read_text()
    except OSError as exc:
        errors.append(f"cannot read {path}: {exc}")
        return {}
    parts = text.split("---", 2)
    if len(parts) != 3 or parts[0].strip():
        errors.append(f"invalid frontmatter in {path}")
        return {}
    try:
        value = yaml.safe_load(parts[1])
    except yaml.YAMLError as exc:
        errors.append(f"cannot parse frontmatter in {path}: {exc}")
        return {}
    if not isinstance(value, dict):
        errors.append(f"expected frontmatter mapping in {path}")
        return {}
    return value


def _file_set(directory: Path) -> set[str]:
    return {path.relative_to(directory).as_posix() for path in directory.rglob("*") if path.is_file()}


def _score(node: dict[str, Any], request: dict[str, Any]) -> int:
    score = 0
    if request.get("target") in node["target_types"]:
        score += 4
    if request.get("goal") in node["goals"]:
        score += 3
    available = set(request.get("prerequisites", []))
    required = set(node["prerequisites"])
    if available & required:
        score += 2
    if required - available:
        score -= 4
    if COST_ORDER[node["cost"]] <= COST_ORDER[request.get("budget", "high")]:
        score += 1
    if set(request.get("exclusions", [])) & set(node["not_for"]):
        score -= 5
    return score


def _rank(nodes: list[dict[str, Any]], request: dict[str, Any]) -> list[str]:
    if not request.get("target") and not request.get("goal"):
        return []
    ranked = sorted(
        nodes,
        key=lambda node: (
            -_score(node, request),
            len(node["target_types"]) + len(node["goals"]),
            COST_ORDER[node["cost"]],
            node["id"],
        ),
    )
    return [node["id"] for node in ranked]


def _verify_routing(nodes: list[dict[str, Any]], errors: list[str]) -> None:
    canonical = 0
    near_neighbor = 0
    near_neighbor_top_one = 0
    for node in nodes:
        for offset in (0, 1):
            request = {
                "target": node["target_types"][offset],
                "goal": node["goals"][offset],
                "prerequisites": node["prerequisites"],
                "budget": node["cost"],
            }
            ranked = _rank(nodes, request)
            canonical += 1
            if not ranked or ranked[0] != node["id"]:
                errors.append(f"canonical route for {node['id']} ranked {ranked[:2]}")

        for key, value in (("target", node["target_types"][0]), ("goal", node["goals"][0])):
            request = {
                key: value,
                "prerequisites": node["prerequisites"],
                "budget": node["cost"],
            }
            ranked = _rank(nodes, request)
            near_neighbor += 1
            if ranked and ranked[0] == node["id"]:
                near_neighbor_top_one += 1
            if node["id"] not in ranked[:2]:
                errors.append(f"near-neighbor route for {node['id']} ranked {ranked[:2]}")

    if canonical != 24 or near_neighbor != 24:
        errors.append(f"expected 24 canonical and 24 near-neighbor routes, got {canonical} and {near_neighbor}")
    if near_neighbor_top_one < 20:
        errors.append(f"near-neighbor top-one coverage is {near_neighbor_top_one}/24; expected at least 20/24")
    insufficient = [{}, {"budget": "low"}]
    insufficient.extend({"prerequisites": [method]} for method in METHOD_IDS[:10])
    if len(insufficient) != 12 or any(_rank(nodes, request) for request in insufficient):
        errors.append("insufficient-context routes must return clarification")


def verify(root: Path) -> list[str]:
    errors: list[str] = []
    skill_root = root / "skills"
    router = skill_root / "security-review-router"
    index_path = router / "references" / "method-index.yaml"
    index = _yaml(index_path, errors)
    nodes = index.get("nodes", [])
    if not isinstance(nodes, list):
        return errors + ["method index nodes must be a list"]

    if tuple(node.get("id") for node in nodes if isinstance(node, dict)) != METHOD_IDS:
        errors.append("method IDs or ordering differ from the v1 twelve-method set")
    if set(index.get("edge_types", {})) != LINK_TYPES:
        errors.append("method index edge types differ from the typed-link contract")
    expected_scoring = {
        "exact_target_match": 4,
        "exact_goal_match": 3,
        "available_prerequisite_match": 2,
        "budget_fit": 1,
        "unmet_prerequisite": -4,
        "explicit_not_for_match": -5,
    }
    scoring = index.get("scoring", {})
    if any(scoring.get(key) != value for key, value in expected_scoring.items()):
        errors.append("routing weights differ from the v1 scoring contract")

    if _file_set(router) != ROUTER_FILES:
        errors.append("router must contain exactly its four declared files")
    router_frontmatter = _frontmatter(router / "SKILL.md", errors)
    if router_frontmatter.get("name") != "security-review-router":
        errors.append("router frontmatter name mismatch")
    if not router_frontmatter.get("description"):
        errors.append("router frontmatter description is required")
    router_agent = _yaml(router / "agents" / "openai.yaml", errors)
    if router_agent.get("policy", {}).get("allow_implicit_invocation") is not True:
        errors.append("router must allow implicit invocation")
    router_text = (router / "SKILL.md").read_text() if (router / "SKILL.md").exists() else ""
    required_router_phrases = {
        "Do not perform a scan",
        "Use at most three local read-only orientation calls",
        "Do not load sibling skills while routing",
        "Keep the response below 150 words",
        "never an execution step",
    }
    for phrase in required_router_phrases:
        if phrase not in router_text:
            errors.append(f"router is missing execution boundary: {phrase}")
    router_words = len(router_text.split()) + (len(index_path.read_text().split()) if index_path.exists() else 0)
    if router_words > 6000:
        errors.append(f"router instructions and index exceed the 6000-word bound: {router_words}")

    by_id = {node.get("id"): node for node in nodes if isinstance(node, dict)}
    link_count = 0
    graph_skills: set[str] = set()
    required_node_fields = {
        "id", "skill", "summary", "target_types", "goals", "prerequisites",
        "cost", "strengths", "not_for", "links",
    }
    for method_id, node in by_id.items():
        if set(node) != required_node_fields:
            errors.append(f"{method_id} node fields differ from the graph contract")
        skill_name = node.get("skill", "")
        graph_skills.add(skill_name)
        method_dir = skill_root / skill_name
        if _file_set(method_dir) != METHOD_FILES:
            errors.append(f"{skill_name} must contain exactly its four declared files")
        frontmatter = _frontmatter(method_dir / "SKILL.md", errors)
        if frontmatter.get("name") != skill_name:
            errors.append(f"frontmatter name mismatch for {skill_name}")
        if not frontmatter.get("description"):
            errors.append(f"frontmatter description is required for {skill_name}")
        agent = _yaml(method_dir / "agents" / "openai.yaml", errors)
        if agent.get("policy", {}).get("allow_implicit_invocation") is not False:
            errors.append(f"{skill_name} must disable implicit invocation")

        skill_text = (method_dir / "SKILL.md").read_text() if (method_dir / "SKILL.md").exists() else ""
        if "../security-review-router/references/review-contract.md" not in skill_text:
            errors.append(f"{skill_name} does not use the shared review contract")
        if f"Expected cost: {node.get('cost')}" not in skill_text:
            errors.append(f"{skill_name} cost differs from the graph")
        for relative, headings in (("references/l2-playbook.md", L2_HEADINGS), ("references/l3-lineage.md", L3_HEADINGS)):
            path = method_dir / relative
            text = path.read_text() if path.exists() else ""
            missing = sorted(heading for heading in headings if heading not in text)
            if missing:
                errors.append(f"{skill_name} missing headings in {relative}: {', '.join(missing)}")

        links = node.get("links", {})
        if set(links) != LINK_TYPES:
            errors.append(f"{method_id} links differ from the typed-link contract")
            continue
        for link_type, destinations in links.items():
            if len(destinations) != len(set(destinations)):
                errors.append(f"duplicate {link_type} edge from {method_id}")
            link_count += len(destinations)
            for destination in destinations:
                if destination not in by_id:
                    errors.append(f"unresolved {link_type} edge: {method_id} -> {destination}")
                elif link_type in SYMMETRIC_LINK_TYPES and method_id not in by_id[destination]["links"][link_type]:
                    errors.append(f"non-reciprocal {link_type} edge: {method_id} -> {destination}")

    method_directories = {path.name for path in skill_root.glob("security-method-*") if path.is_dir()}
    if method_directories != graph_skills:
        errors.append("security method directories differ from graph skill names")
    if link_count != EXPECTED_LINKS:
        errors.append(f"expected {EXPECTED_LINKS} resolved links, found {link_count}")
    if len(by_id) == len(METHOD_IDS):
        _verify_routing(list(by_id.values()), errors)
    return errors


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    errors = verify(root)
    if errors:
        for error in errors:
            print(f"skill-integrity: {error}", file=sys.stderr)
        return 1
    print(f"skill-integrity: ok ({len(METHOD_IDS)} methods, {EXPECTED_LINKS} links, 60 routing cases)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
