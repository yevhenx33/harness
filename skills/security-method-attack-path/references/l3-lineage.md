# L3 Lineage: Attack-Path Analysis

## Pressure

Weakness labels and isolated alerts overstate risk when attacker influence cannot reach a meaningful outcome. Attack graphs modeled prerequisite chains at system scale, while taint analysis modeled source-to-sink influence in code.

## Method response

The method combines those ideas into an explicit ordered attacker story whose edges can be supported or falsified individually.

## Current shape

Modern analysis crosses code, identity, storage, queues, infrastructure, and recovery. It calibrates severity only after the terminal consequence and prerequisites are known.

## Inherited strengths

- Converts plausible alerts into testable paths.
- Identifies the narrowest guard that can break exploitation.
- Connects threat scenarios to exploit validation.

## Known failure modes

- Treating potential data flow as reachable execution.
- Omitting attacker prerequisites or deployment constraints.
- Skipping async, serialization, or identity edges.
- Assigning impact from the vulnerability class rather than the sink.

## Primary anchors

- [NIST IR 7788: Probabilistic Attack Graphs](https://csrc.nist.gov/pubs/ir/7788/final)
- [NIST: Mapping Evidence Graphs to Attack Graphs](https://www.nist.gov/publications/mapping-evidence-graphs-attack-graphs)
- [OWASP Taint Analysis](https://mas.owasp.org/MASTG/techniques/android/MASTG-TECH-0108)
- [NIST Zero-Day Attack Paths](https://www.nist.gov/publications/towards-probabilistic-identification-zero-day-attack-paths)
