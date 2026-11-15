//! The first section is the portable core. Named sections are explicit opt-ins.
use std::collections::{BTreeMap, BTreeSet};

pub const CORE: &str = "core";
pub const PATH: &str = "contracts/skill-activation.txt";
type Profiles = BTreeMap<String, BTreeSet<String>>;

pub fn valid_name(value: &str) -> bool {
    value.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && value.len() <= 80
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

pub fn parse(bytes: &[u8]) -> Result<Profiles, Box<dyn std::error::Error>> {
    let mut profiles = BTreeMap::from([(CORE.to_owned(), BTreeSet::new())]);
    let mut assigned = BTreeSet::new();
    let mut current = CORE;
    for line in std::str::from_utf8(bytes)?.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(section) = line
            .strip_prefix('[')
            .and_then(|line| line.strip_suffix(']'))
        {
            if !valid_name(section)
                || profiles
                    .insert(section.to_owned(), BTreeSet::new())
                    .is_some()
            {
                return Err(format!("invalid or duplicate skill profile: {section}").into());
            }
            current = section;
        } else {
            if !valid_name(line) || !assigned.insert(line) {
                return Err(format!("invalid or multiply assigned skill: {line}").into());
            }
            profiles.get_mut(current).unwrap().insert(line.to_owned());
        }
    }
    if profiles.values().any(BTreeSet::is_empty) {
        return Err("skill profiles must be nonempty".into());
    }
    Ok(profiles)
}

pub fn select(
    profiles: &Profiles,
    name: &str,
) -> Result<BTreeSet<String>, Box<dyn std::error::Error>> {
    let mut selected = profiles[CORE].clone();
    if name != CORE {
        selected.extend(
            profiles
                .get(name)
                .ok_or_else(|| format!("unknown skill profile: {name}"))?
                .iter()
                .cloned(),
        );
    }
    Ok(selected)
}

pub fn transition(
    previous: &Profiles,
    current: &Profiles,
    profile: &str,
) -> Result<(BTreeSet<String>, BTreeSet<String>), Box<dyn std::error::Error>> {
    let selected = select(current, profile)?;
    // An older release without this opt-in profile still supplied the core.
    let old = select(previous, profile).unwrap_or_else(|_| previous[CORE].clone());
    let added = selected.difference(&old).cloned().collect();
    Ok((selected, added))
}
