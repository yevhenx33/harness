// Read-only replay adapter for the retained Router V5 stage 021 format.
use crate::run::{self, Case, Execution, Observed, Outcome, Plan, Report};
use crate::{regular, Result, INPUT_LIMIT};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::{self, Read};
use std::path::{Component, Path};

const EXE_LIMIT: u64 = 128 * 1024 * 1024;

#[derive(Serialize)]
pub(crate) struct Control {
    name: &'static str,
    expected: &'static str,
    result: Report,
}

fn read(dir: &Path, name: &str) -> Result<String> {
    let mut data = String::new();
    regular(&dir.join(name))?
        .take(INPUT_LIMIT + 1)
        .read_to_string(&mut data)?;
    if data.len() as u64 > INPUT_LIMIT {
        return Err("Router input exceeds 1 MiB".into());
    }
    Ok(data)
}

fn lookup<'a>(table: &'a str, key: &str) -> Result<&'a str> {
    let mut matches = table.lines().filter_map(|line| {
        let (name, value) = line.split_once('\t')?;
        (name == key).then_some(value)
    });
    let value = matches.next().ok_or("missing Router evidence field")?;
    if matches.next().is_some() {
        return Err("ambiguous Router evidence field".into());
    }
    Ok(value)
}

fn binary(dir: &Path, target: &str) -> Result<String> {
    let inventory = read(dir, "router-ci-context/bundle/tests.tsv")?;
    let path = Path::new(lookup(&inventory, target)?);
    if !path.starts_with("deps")
        || path.components().count() != 2
        || !path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err("unsafe executable inventory member".into());
    }
    let mut file = regular(&dir.join("router-ci-context/bundle").join(path))?.take(EXE_LIMIT + 1);
    let mut sha = Sha256::new();
    if io::copy(&mut file, &mut sha)? > EXE_LIMIT {
        return Err("Router executable exceeds 128 MiB".into());
    }
    Ok(format!("{:x}", sha.finalize()))
}

// This stage retains repeated group summaries because the recipe prints them
// again. The first complete libtest group is the server auth unit group.
fn auth_group(log: &str) -> Result<(Vec<Case>, Outcome)> {
    let mut lines = log.lines();
    let count: usize = lines
        .find_map(|line| line.strip_prefix("running "))
        .and_then(|line| line.strip_suffix(" tests"))
        .ok_or("missing retained auth group start")?
        .parse()?;
    let mut cases = Vec::new();
    let mut summary = None;
    for line in lines {
        if let Some(status) = line.strip_prefix("test result: ") {
            summary = Some(status);
            break;
        }
        if let Some((name, status)) = line
            .strip_prefix("test ")
            .and_then(|line| line.rsplit_once(" ... "))
        {
            if !name.starts_with("auth::tests::") {
                return Err("unexpected case in retained auth group".into());
            }
            let outcome = match status {
                "ok" => Outcome::Passed,
                "FAILED" => Outcome::Failed,
                s if s == "ignored" || s.starts_with("ignored, ") => Outcome::Skipped,
                _ => return Err("unsupported case result in retained auth group".into()),
            };
            cases.push(Case {
                id: name.into(),
                outcome,
            });
        }
    }
    let passed = cases
        .iter()
        .filter(|case| case.outcome == Outcome::Passed)
        .count();
    let failed = cases
        .iter()
        .filter(|case| case.outcome == Outcome::Failed)
        .count();
    let skipped = cases
        .iter()
        .filter(|case| case.outcome == Outcome::Skipped)
        .count();
    let result = if failed == 0 { "ok" } else { "FAILED" };
    let expected = format!("{result}. {passed} passed; {failed} failed; {skipped} ignored;");
    if cases.len() != count || !summary.is_some_and(|s| s.starts_with(&expected)) {
        return Err("incomplete or inconsistent retained auth group".into());
    }
    Ok((
        cases,
        if failed == 0 {
            Outcome::Passed
        } else {
            Outcome::Failed
        },
    ))
}

pub(crate) fn replay(plan: &Plan, dir: &Path) -> Result<Vec<Control>> {
    // The scope is historical stage 021. Authenticate neither the archive nor a
    // newly run test: compare the stage, source and bundle records from its owner.
    let inputs = read(dir, "inputs.tsv")?;
    let source = lookup(&inputs, "source")?;
    let fingerprint = lookup(&inputs, "fingerprint")?;
    let candidate_sha = read(dir, "router-ci/candidate.sha256")?;
    let candidate_sha = candidate_sha.trim();
    let receipt = read(dir, "router-results/router-receipt-routing/routing.receipt")?;
    if receipt.trim_end() != format!("{source}|{candidate_sha}|routing")
        || !run::digest(candidate_sha)
    {
        return Err("Router source and routing receipt do not agree".into());
    }
    let pass = read(dir, "PASS.receipt")?;
    if pass.lines().next() != Some("router-local-v1")
        || lookup(&pass, "source")? != source
        || lookup(&pass, "fingerprint")? != fingerprint
        || lookup(&pass, "bundle")? != candidate_sha
        || lookup(&pass, "mode")? != "development"
    {
        return Err("Router candidate differs from its retained development receipt".into());
    }
    let stages = read(dir, "steps.tsv")?;
    let stage = lookup(&stages, "021")?;
    if !matches!(
        stage.split('\t').collect::<Vec<_>>().as_slice(),
        [
            "Verify authenticated public acquisition and packaged administration",
            "0",
            _
        ]
    ) {
        return Err("missing successful Router stage 021".into());
    }
    let recipe = read(dir, "021.bash")?;
    if !recipe.contains("for group in auth::tests::")
        || !recipe.contains("router-ci test --bin rpc-router")
    {
        return Err(
            "Router retained recipe does not select the expected auth group and role".into(),
        );
    }
    for id in plan
        .guarantees
        .iter()
        .flat_map(|guarantee| &guarantee.cases)
    {
        let marker = format!("grep -Fqx 'test {id} ... ok'");
        if !recipe
            .lines()
            .any(|line| line.trim_start().starts_with(&marker))
        {
            return Err("planned case is absent from the retained recipe".into());
        }
    }
    let recipe_sha = format!("{:x}", Sha256::digest(recipe));
    let (cases, outcome) = auth_group(&read(dir, "021.log")?)?;
    if outcome != Outcome::Passed {
        return Err("retained auth result conflicts with successful stage".into());
    }
    let execution = Execution {
        candidate: format!("{source}:{candidate_sha}"),
        package: "rpc-router".into(),
        target: "bin:rpc-router".into(),
        configuration: format!("latency-fixture:auth::tests::{fingerprint}:{recipe_sha}"),
        executable_sha256: binary(dir, "rpc-router")?,
        run: format!(
            "{}:021:auth",
            dir.file_name()
                .and_then(|name| name.to_str())
                .ok_or("invalid run directory")?
        ),
    };
    let valid = Observed {
        schema: 1,
        execution,
        outcome,
        cases,
    };
    let mut wrong = valid.clone();
    wrong.execution.target = "lib:router-sdk".into();
    wrong.execution.executable_sha256 = binary(dir, "router-sdk")?;
    let mut stale = valid.clone();
    stale.execution.run.push_str(":older-attempt");
    let mut missing = valid.clone();
    let first = plan
        .guarantees
        .first()
        .and_then(|row| row.cases.first())
        .ok_or("empty Router plan")?;
    missing.cases.retain(|case| &case.id != first);

    let controls = [
        ("retained_server_run", "verified", valid),
        ("wrong_binary_same_case", "unverified", wrong),
        ("stale_execution", "unverified", stale),
        ("missing_case_other_cases_green", "unverified", missing),
    ];
    controls
        .into_iter()
        .map(|(name, expected, observed)| {
            let result = run::check(plan, &observed)?;
            if result.status != expected {
                return Err(
                    format!("Router replay control {name} did not produce {expected}").into(),
                );
            }
            Ok(Control {
                name,
                expected,
                result,
            })
        })
        .collect()
}
