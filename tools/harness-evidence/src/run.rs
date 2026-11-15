use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Execution {
    pub(crate) candidate: String,
    pub(crate) package: String,
    pub(crate) target: String,
    pub(crate) configuration: String,
    pub(crate) executable_sha256: String,
    pub(crate) run: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Plan {
    pub(crate) schema: u32,
    pub(crate) expected: Execution,
    pub(crate) guarantees: Vec<Guarantee>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Guarantee {
    pub(crate) id: String,
    owner: String,
    source: String,
    outcome: String,
    pub(crate) cases: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Observed {
    pub(crate) schema: u32,
    pub(crate) execution: Execution,
    pub(crate) outcome: Outcome,
    pub(crate) cases: Vec<Case>,
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Outcome {
    Passed,
    Failed,
    Skipped,
    Incomplete,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Case {
    pub(crate) id: String,
    pub(crate) outcome: Outcome,
}

#[derive(Serialize)]
pub(crate) struct Report {
    pub(crate) status: &'static str,
    observed_run_outcome: Outcome,
    mismatched_fields: Vec<&'static str>,
    guarantees: Vec<GuaranteeResult>,
}

#[derive(Serialize)]
struct GuaranteeResult {
    id: String,
    owner: String,
    source: String,
    outcome: String,
    status: &'static str,
    failed: Vec<String>,
    skipped: Vec<String>,
    incomplete: Vec<String>,
    missing: Vec<String>,
}

fn present(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 512 && !value.chars().any(char::is_control)
}

pub(crate) fn digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

impl Execution {
    fn valid(&self) -> bool {
        digest(&self.executable_sha256)
            && [
                &self.candidate,
                &self.package,
                &self.target,
                &self.configuration,
                &self.run,
            ]
            .into_iter()
            .all(|value| present(value))
    }

    fn differences(&self, other: &Self) -> Vec<&'static str> {
        [
            ("candidate", &self.candidate, &other.candidate),
            ("package", &self.package, &other.package),
            ("target", &self.target, &other.target),
            ("configuration", &self.configuration, &other.configuration),
            (
                "executable_sha256",
                &self.executable_sha256,
                &other.executable_sha256,
            ),
            ("run", &self.run, &other.run),
        ]
        .into_iter()
        .filter_map(|(field, expected, observed)| (expected != observed).then_some(field))
        .collect()
    }
}

pub(crate) fn check(plan: &Plan, observed: &Observed) -> Result<Report> {
    if plan.schema != 1 || observed.schema != 1 {
        return Err("unsupported run evidence schema".into());
    }
    if !plan.expected.valid() || !observed.execution.valid() || plan.guarantees.is_empty() {
        return Err("invalid execution identity or empty guarantee plan".into());
    }
    let mut guarantees = BTreeSet::new();
    for guarantee in &plan.guarantees {
        let fields = [
            &guarantee.id,
            &guarantee.owner,
            &guarantee.source,
            &guarantee.outcome,
        ];
        let mut cases = BTreeSet::new();
        if !fields.into_iter().all(|value| present(value))
            || !guarantees.insert(&guarantee.id)
            || guarantee.cases.is_empty()
            || !guarantee
                .cases
                .iter()
                .all(|id| present(id) && cases.insert(id))
        {
            return Err("invalid guarantee or duplicate guarantee/case identity".into());
        }
    }
    let mut cases = BTreeMap::new();
    for case in &observed.cases {
        if !present(&case.id) || cases.insert(case.id.as_str(), case.outcome).is_some() {
            return Err("invalid or duplicate observed case identity".into());
        }
    }

    if observed.outcome == Outcome::Passed
        && cases.values().any(|outcome| *outcome == Outcome::Failed)
    {
        return Err("passed run contradicts a failed case".into());
    }

    if observed.outcome == Outcome::Skipped
        && cases
            .values()
            .any(|outcome| matches!(outcome, Outcome::Passed | Outcome::Failed))
    {
        return Err("skipped run contradicts an executed case".into());
    }
    let mismatched_fields = plan.expected.differences(&observed.execution);
    let matching = mismatched_fields.is_empty();
    let mut results = Vec::new();
    for guarantee in &plan.guarantees {
        let mut result = GuaranteeResult {
            id: guarantee.id.clone(),
            owner: guarantee.owner.clone(),
            source: guarantee.source.clone(),
            outcome: guarantee.outcome.clone(),
            status: "unverified",
            failed: Vec::new(),
            skipped: Vec::new(),
            incomplete: Vec::new(),
            missing: Vec::new(),
        };
        if matching {
            for id in &guarantee.cases {
                match cases.get(id.as_str()) {
                    Some(Outcome::Passed) => {}
                    Some(Outcome::Failed) => result.failed.push(id.clone()),
                    Some(Outcome::Skipped) => result.skipped.push(id.clone()),
                    Some(Outcome::Incomplete) => result.incomplete.push(id.clone()),
                    None => result.missing.push(id.clone()),
                }
            }
            if !result.failed.is_empty() {
                result.status = "failed";
            } else if result.skipped.is_empty()
                && result.incomplete.is_empty()
                && result.missing.is_empty()
                && observed.outcome == Outcome::Passed
            {
                result.status = "verified";
            }
        }
        results.push(result);
    }
    let status = if results.iter().any(|row| row.status == "failed") {
        "failed"
    } else if results.iter().all(|row| row.status == "verified") {
        "verified"
    } else {
        "unverified"
    };
    Ok(Report {
        status,
        observed_run_outcome: observed.outcome,
        mismatched_fields,
        guarantees: results,
    })
}
