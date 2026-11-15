use crate::{regular, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::{self, File, Metadata};
use std::io::{self, BufRead, BufReader, Read};
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};
use std::time::{Duration, Instant};

const MIB: u64 = 1024 * 1024;
const MAX_EVENTS: usize = 16_384;
const MAX_RECORD: u64 = 8 * MIB;
const MAX_REFERENCE_PATH: usize = 64;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Manifest {
    schema: u32,
    limits: Limits,
    skills: BTreeSet<String>,
    chats: Vec<Chat>,
    references: References,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Limits {
    seconds: u64,
    stored_bytes: u64,
    decoded_bytes: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Chat {
    id: String,
    fragments: Vec<Fragment>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Fragment {
    path: PathBuf,
    member: Option<String>,
    native_events: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct References {
    complete: bool,
    roots: Vec<String>,
    edges: Vec<[String; 2]>,
}

#[derive(Default)]
struct Budget {
    used: Cell<u64>,
    stop: Cell<Option<&'static str>>,
}

struct Meter<'a, R> {
    input: R,
    budget: &'a Budget,
    limit: u64,
    deadline: Instant,
    reason: &'static str,
}

impl<R: Read> Read for Meter<'_, R> {
    fn read(&mut self, target: &mut [u8]) -> io::Result<usize> {
        if target.is_empty() {
            return Ok(0);
        }
        if Instant::now() >= self.deadline {
            self.budget.stop.set(Some("time_limit"));
            return Err(io::Error::new(io::ErrorKind::Other, "time bound reached"));
        }
        let remaining = self.limit - self.budget.used.get();
        let len = remaining.min(target.len() as u64) as usize;
        if len == 0 {
            self.budget.stop.set(Some(self.reason));
            return Err(io::Error::new(io::ErrorKind::Other, "byte bound reached"));
        }
        // A nested reader can borrow the same budget. Reserve capacity before
        // it runs, then account only for the bytes each layer delivered.
        self.budget.used.set(self.budget.used.get() + len as u64);
        let result = self.input.read(&mut target[..len]);
        let size = result.as_ref().copied().unwrap_or(0);
        self.budget
            .used
            .set(self.budget.used.get() - len as u64 + size as u64);
        result
    }
}

fn unzstd<R: Read>(input: R) -> io::Result<impl Read> {
    let mut decoder = zstd::stream::read::Decoder::new(input)?;
    decoder.window_log_max(27)?;
    Ok(decoder)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum Kind {
    Request,
    PathReference,
    NativeLoad,
}

#[derive(Default, Serialize)]
struct Count {
    identified_events: usize,
    unidentified_observations: usize,
    conflicting_identities: usize,
    observations_without_turn: usize,
    known_turns: usize,
    #[serde(skip)]
    turns: BTreeMap<String, usize>,
}

enum Turn {
    Unknown,
    Known(String),
    Conflicting,
}

impl Count {
    fn add_turn(&mut self, id: &str) {
        *self.turns.entry(id.to_string()).or_default() += 1;
        self.known_turns = self.turns.len();
    }

    fn remove_turn(&mut self, id: &str) {
        if let std::collections::btree_map::Entry::Occupied(mut entry) =
            self.turns.entry(id.to_string())
        {
            *entry.get_mut() -= 1;
            if *entry.get() == 0 {
                entry.remove();
            }
        }
        self.known_turns = self.turns.len();
    }
}

#[derive(Default)]
struct Counts {
    // Dedup keys and turn IDs belong to a single chat. IDs are never guessed
    // from the contents or cross-chat lineage.
    seen: BTreeMap<(Kind, String, String, usize), Turn>,
    native_names: BTreeMap<(String, usize), Option<String>>,
    buckets: BTreeMap<(Kind, String), Count>,
    observations: usize,
}

impl Counts {
    fn check_native_name(
        &mut self,
        skill: &str,
        event: &str,
        index: usize,
    ) -> std::result::Result<bool, &'static str> {
        let key = (event.to_string(), index);
        let previous = match self.native_names.entry(key) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(Some(skill.to_string()));
                return Ok(true);
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => match entry.get() {
                Some(previous) if previous == skill => return Ok(false),
                Some(_) => entry.insert(None).expect("a prior name"),
                None => return Err("conflicting skill for copied native event"),
            },
        };
        if let Some(turn) =
            self.seen
                .remove(&(Kind::NativeLoad, previous.clone(), event.to_string(), index))
        {
            let count = self
                .buckets
                .get_mut(&(Kind::NativeLoad, previous))
                .expect("observed native skill");
            count.identified_events -= 1;
            count.conflicting_identities += 1;
            match turn {
                Turn::Known(turn) => count.remove_turn(&turn),
                Turn::Unknown | Turn::Conflicting => count.observations_without_turn -= 1,
            }
        }
        Err("conflicting skill for copied native event")
    }

    fn add(
        &mut self,
        kind: Kind,
        skill: &str,
        event: Option<&str>,
        index: usize,
        turn: Option<&str>,
        selected: bool,
    ) -> std::result::Result<(), &'static str> {
        if kind == Kind::NativeLoad {
            let new_slot = match event {
                Some(event) => self.check_native_name(skill, event, index)?,
                None => false,
            };
            if !selected {
                self.observations += usize::from(new_slot);
                return Ok(());
            }
        }
        let bucket = (kind, skill.to_string());
        if let Some(event) = event {
            let key = (kind, skill.to_string(), event.to_string(), index);
            if let Some(previous) = self.seen.get_mut(&key) {
                let count = self
                    .buckets
                    .get_mut(&bucket)
                    .expect("existing event has a count");
                match (&*previous, turn) {
                    (Turn::Known(old), Some(new)) if old != new => {
                        count.remove_turn(old);
                        count.observations_without_turn += 1;
                        *previous = Turn::Conflicting;
                        return Err("conflicting turn for copied event");
                    }
                    (Turn::Unknown, Some(new)) => {
                        *previous = Turn::Known(new.to_string());
                        count.observations_without_turn -= 1;
                        count.add_turn(new);
                    }
                    (Turn::Conflicting, _) => return Err("conflicting turn for copied event"),
                    _ => {}
                }
                return Ok(());
            }
            self.seen.insert(
                key,
                turn.map_or(Turn::Unknown, |id| Turn::Known(id.to_string())),
            );
        }
        self.observations += 1;
        let count = self.buckets.entry(bucket).or_default();
        if event.is_some() {
            count.identified_events += 1;
        } else {
            count.unidentified_observations += 1;
        }
        if let Some(turn) = turn {
            count.add_turn(turn);
        } else {
            count.observations_without_turn += 1;
        }
        Ok(())
    }
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_:.@".contains(&b))
}

fn identifier(value: &Value) -> Option<&str> {
    value.as_str().filter(|id| valid_id(id))
}

fn requests(text: &str) -> impl Iterator<Item = &str> {
    text.match_indices('$').filter_map(move |(index, _)| {
        let name = &text[index + 1..];
        let end = name
            .find(|c: char| !(c.is_ascii_alphanumeric() || "-_:".contains(c)))
            .unwrap_or(name.len());
        let boundary = text[..index].chars().next_back();
        (!matches!(boundary, Some(c) if c.is_ascii_alphanumeric() || c == '\\' || c == '$'))
            .then_some(&name[..end])
    })
}

fn path_references(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| c.is_whitespace() || c == '\x60' || "'\"()[]{};,|<>\\".contains(c))
        .filter_map(|token| {
            let path = token
                .rsplit_once("/skills/")
                .map(|(_, path)| path)
                .or_else(|| token.strip_prefix("skills/"))?;
            path.strip_suffix("/SKILL.md")
        })
}

fn collect_references<'a>(
    arguments: &'a Value,
    skills: &BTreeSet<String>,
    found: &mut BTreeSet<&'a str>,
) {
    match arguments {
        Value::String(text) => {
            found.extend(path_references(text).filter(|skill| skills.contains(*skill)))
        }
        Value::Array(values) => {
            for value in values {
                collect_references(value, skills, found);
            }
        }
        Value::Object(fields) => {
            for value in fields.values() {
                collect_references(value, skills, found);
            }
        }
        _ => {}
    }
}

fn metadata_array(value: &Value) -> std::result::Result<Option<&[Value]>, &'static str> {
    if value.is_null() {
        return Ok(None);
    }
    value
        .as_array()
        .map(|array| Some(array.as_slice()))
        .ok_or("invalid content metadata")
}

const NATIVE_SELECTION: &str = "skills.selected_skill_instructions";

fn native_name(text: &str) -> Option<&str> {
    let mut lines = text.lines();
    if lines.next()? != "<skill>" {
        return None;
    }
    lines
        .next()?
        .strip_prefix("<name>")?
        .strip_suffix("</name>")
        .filter(|name| valid_id(name))
}

struct Lines<'a> {
    id: &'a str,
    skills: &'a BTreeSet<String>,
    counts: &'a mut Counts,
    event_budget: &'a mut usize,
}

impl Lines<'_> {
    fn observe(
        &mut self,
        kind: Kind,
        skill: &str,
        event: Option<&str>,
        index: usize,
        turn: Option<&str>,
    ) -> std::result::Result<(), &'static str> {
        let selected = self.skills.contains(skill);
        if selected || kind == Kind::NativeLoad {
            if *self.event_budget == 0 {
                return Err("event limit");
            }
            let before = self.counts.observations;
            self.counts.add(kind, skill, event, index, turn, selected)?;
            *self.event_budget -= self.counts.observations - before;
        }
        Ok(())
    }

    fn event(&mut self, row: &Value) -> std::result::Result<(), &'static str> {
        let payload = &row["payload"];
        if row["type"] == "session_meta" {
            return if payload["id"] == self.id {
                Ok(())
            } else {
                Err("transcript chat identity mismatch")
            };
        }
        if row["type"] != "response_item" {
            return Ok(());
        }
        let kind = payload["type"].as_str().ok_or("invalid response item")?;
        if !matches!(kind, "message" | "function_call" | "custom_tool_call") {
            return Ok(());
        }

        let metadata = &payload["internal_chat_message_metadata_passthrough"];
        if !metadata.is_null() && !metadata.is_object() {
            return Err("invalid response item metadata");
        }
        let metadata_turn = identifier(&metadata["turn_id"]);
        let payload_turn = identifier(&payload["turn_id"]);
        if metadata_turn
            .zip(payload_turn)
            .is_some_and(|(left, right)| left != right)
        {
            return Err("conflicting turn IDs on response item");
        }
        let turn = metadata_turn.or(payload_turn);

        if kind != "message" {
            let event = identifier(&payload["call_id"]).or_else(|| identifier(&payload["id"]));
            let field = if kind == "function_call" {
                "arguments"
            } else {
                "input"
            };
            let arguments = &payload[field];
            if !(arguments.is_string() || arguments.is_object() || arguments.is_array()) {
                return Err("invalid tool call arguments");
            }
            let mut references = BTreeSet::new();
            collect_references(arguments, self.skills, &mut references);
            for skill in references {
                self.observe(Kind::PathReference, skill, event, 0, turn)?;
            }
            return Ok(());
        }

        let role = payload["role"].as_str().ok_or("invalid message role")?;
        let kinds = metadata_array(&metadata["content_item_kinds"])?;
        let sources = metadata_array(&metadata["content_item_metadata"])?;
        if role != "user" {
            let has_selection = kinds
                .iter()
                .flat_map(|items| items.iter())
                .any(|value| value == NATIVE_SELECTION);
            return if has_selection {
                Err("unsupported role for native skill evidence")
            } else {
                Ok(())
            };
        }

        let event = identifier(&payload["id"]);
        let contents = payload["content"]
            .as_array()
            .ok_or("invalid message content")?;
        if kinds.is_some_and(|k| k.len() != contents.len())
            || sources.is_some_and(|s| s.len() != contents.len())
        {
            return Err("unsupported content metadata alignment");
        }
        for (index, content) in contents.iter().enumerate() {
            let kind = kinds.and_then(|items| items.get(index));
            let provenance = sources
                .and_then(|items| items.get(index))
                .map(|item| &item["provenance"]["type"]);
            let selected = kind.is_some_and(|kind| kind == NATIVE_SELECTION);
            let native = provenance.is_some_and(|value| value == "skills");
            if selected {
                if !native {
                    return Err("uncorroborated native skill evidence");
                }
                let skill = content["text"]
                    .as_str()
                    .and_then(native_name)
                    .ok_or("unsupported native skill envelope")?;
                self.observe(Kind::NativeLoad, skill, event, index, turn)?;
            } else if native {
                continue;
            } else if let Some(text) = content["text"].as_str() {
                for skill in requests(text)
                    .filter(|skill| self.skills.contains(*skill))
                    .collect::<BTreeSet<_>>()
                {
                    self.observe(Kind::Request, skill, event, index, turn)?;
                }
            } else if content["type"] == "input_text" {
                return Err("invalid user text");
            }
        }
        Ok(())
    }

    fn fragment(&mut self, input: impl Read) -> FragmentStats {
        let mut reader = BufReader::new(input);
        let mut stats = FragmentStats::new("partial");
        let mut first = true;
        let mut line = Vec::new();
        loop {
            line.clear();
            let result = reader
                .by_ref()
                .take(MAX_RECORD + 1)
                .read_until(b'\n', &mut line);
            stats.transcript_bytes_observed += line.len() as u64;
            match result {
                Ok(0) => {
                    stats.status = if first { "unsupported" } else { "complete" };
                    return stats;
                }
                Ok(n) if n as u64 <= MAX_RECORD && line.last() == Some(&b'\n') => {}
                Ok(_) => return stats.with_issue("incomplete or oversized transcript record"),
                Err(_) => return stats.with_issue("transcript read stopped"),
            }
            let Ok(row) = serde_json::from_slice::<Value>(&line) else {
                return stats.with_issue("invalid transcript JSON");
            };
            if !row["type"].is_string() {
                stats.status = if first { "unsupported" } else { "partial" };
                return stats.with_issue("unrecognized transcript record");
            }
            if first {
                if row["type"] != "session_meta" || row["payload"]["id"] != self.id {
                    stats.status = "unsupported";
                    return stats.with_issue("missing matching session metadata");
                }
                first = false;
            }
            if let Err(issue) = self.event(&row) {
                return stats.with_issue(issue);
            }
            if row["type"] == "response_item"
                && matches!(
                    row["payload"]["type"].as_str(),
                    Some("function_call_output" | "custom_tool_call_output")
                )
            {
                stats.tool_output_bytes_observed += line.len() as u64;
            }
        }
    }
}

#[derive(Clone, Serialize)]
struct FragmentStats {
    status: &'static str,
    issue: Option<&'static str>,
    transcript_bytes_observed: u64,
    tool_output_bytes_observed: u64,
}

impl FragmentStats {
    fn new(status: &'static str) -> Self {
        Self {
            status,
            issue: None,
            transcript_bytes_observed: 0,
            tool_output_bytes_observed: 0,
        }
    }

    fn with_issue(mut self, issue: &'static str) -> Self {
        self.issue = Some(issue);
        self
    }
}

fn codec(path: &Path) -> Option<bool> {
    let name = path.file_name()?.to_str()?;
    if name.ends_with(".jsonl.zst") {
        Some(true)
    } else if name.ends_with(".jsonl") {
        Some(false)
    } else {
        None
    }
}

struct Scan<'a> {
    manifest: &'a Manifest,
    deadline: Instant,
    stored: &'a Budget,
    decoded: &'a Budget,
    event_budget: usize,
    counts: Vec<Counts>,
}

type Address = (usize, usize);

impl Scan<'_> {
    fn meter<'a, R: Read>(
        &self,
        input: R,
        budget: &'a Budget,
        limit: u64,
        reason: &'static str,
    ) -> Meter<'a, R> {
        Meter {
            input,
            budget,
            limit,
            deadline: self.deadline,
            reason,
        }
    }

    fn lines(&mut self, address: Address, input: impl Read, compressed: bool) -> FragmentStats {
        let chat = address.0;
        let meter = |input| Meter {
            input,
            budget: self.decoded,
            limit: self.manifest.limits.decoded_bytes,
            deadline: self.deadline,
            reason: "decoded_byte_limit",
        };
        // Use an erased reader only at this encoding boundary.
        let input: Box<dyn Read + '_> = if compressed {
            match unzstd(input) {
                Ok(decoder) => Box::new(decoder),
                Err(_) => return FragmentStats::new("partial"),
            }
        } else {
            Box::new(input)
        };
        Lines {
            id: &self.manifest.chats[chat].id,
            skills: &self.manifest.skills,
            counts: &mut self.counts[chat],
            event_budget: &mut self.event_budget,
        }
        .fragment(meter(input))
    }

    fn source(
        &mut self,
        path: &Path,
        file: File,
        targets: &BTreeMap<Option<String>, Address>,
        stats: &mut [Vec<FragmentStats>],
    ) -> bool {
        let input = self.meter(
            file,
            self.stored,
            self.manifest.limits.stored_bytes,
            "stored_byte_limit",
        );
        let complete = if let Some(address) = targets.get(&None).copied() {
            let result = codec(path).map_or_else(
                || FragmentStats::new("unsupported"),
                |compressed| self.lines(address, input, compressed),
            );
            let complete = result.status == "complete";
            stats[address.0][address.1] = result;
            complete
        } else if path.to_string_lossy().ends_with(".tar.zst") {
            match unzstd(input) {
                Ok(decoder) => self.tar(decoder, targets, stats),
                Err(_) => false,
            }
        } else if path.extension().is_some_and(|ext| ext == "tar") {
            self.tar(input, targets, stats)
        } else {
            for &(chat, fragment) in targets.values() {
                stats[chat][fragment].status = "unsupported";
            }
            false
        };
        complete
    }

    fn tar(
        &mut self,
        input: impl Read,
        targets: &BTreeMap<Option<String>, Address>,
        stats: &mut [Vec<FragmentStats>],
    ) -> bool {
        // Charge both outer tar work and any nested expansion.
        let input = self.meter(
            input,
            self.decoded,
            self.manifest.limits.decoded_bytes,
            "decoded_byte_limit",
        );
        let mut archive = tar::Archive::new(input);
        let mut seen = BTreeSet::new();
        let mut long_name = None;
        let Ok(entries) = archive.entries() else {
            return false;
        };
        for entry in entries.raw(true) {
            let Ok(mut entry) = entry else {
                return false;
            };
            let entry_type = entry.header().entry_type();
            if entry_type.is_gnu_longname() {
                let mut name = Vec::new();
                if long_name.is_some()
                    || entry.take(4097).read_to_end(&mut name).is_err()
                    || name.len() > 4096
                    || name.pop() != Some(0)
                {
                    return false;
                }
                let Ok(name) = String::from_utf8(name) else {
                    return false;
                };
                long_name = Some(name);
                continue;
            }
            if entry_type.is_pax_local_extensions()
                || entry_type.is_pax_global_extensions()
                || entry_type.is_gnu_sparse()
            {
                return false;
            }
            let name = match long_name.take() {
                Some(name) => name,
                None => match String::from_utf8(entry.path_bytes().into_owned()) {
                    Ok(name) => name,
                    Err(_) => return false,
                },
            };
            let format = codec(Path::new(&name));
            let key = Some(name);
            let Some(address) = targets.get(&key).copied() else {
                continue;
            };
            if !seen.insert(key) {
                return false;
            }
            let result = match (entry_type.is_file(), format) {
                (true, Some(true)) => self.lines(address, entry, true),
                (true, Some(false)) => Lines {
                    id: &self.manifest.chats[address.0].id,
                    skills: &self.manifest.skills,
                    counts: &mut self.counts[address.0],
                    event_budget: &mut self.event_budget,
                }
                .fragment(&mut entry),
                _ => FragmentStats::new("unsupported"),
            };
            stats[address.0][address.1] = result;
        }

        // The iterator stops at the first zero header. Check the second zero
        // block and any padding, and reach codec EOF, before crediting coverage.
        let mut input = archive.into_inner();
        let mut block = [0; 8192];
        let mut padding = 0;
        loop {
            match input.read(&mut block) {
                Ok(0) => return padding >= 512 && long_name.is_none(),
                Ok(n) if block[..n].iter().all(|byte| *byte == 0) => padding += n,
                _ => return false,
            }
        }
    }
}

#[derive(Serialize)]
pub(crate) struct Report {
    registered_chats: usize,
    complete_chats: usize,
    partial_chats: usize,
    unreadable_chats: usize,
    unsupported_chats: usize,
    native_load_eligible_chats: usize,
    stored_bytes_read: u64,
    decoded_bytes_processed: u64,
    allocated_bytes_observed: u64,
    sources_with_unknown_allocation: usize,
    stop_reason: Option<&'static str>,
    sources: Vec<SourceReport>,
    chats: Vec<ChatReport>,
}

#[derive(Serialize)]
struct ChatReport {
    id: String,
    status: &'static str,
    native_load_eligible: bool,
    retention_reference: &'static str,
    reference_path: Vec<String>,
    reference_path_truncated: bool,
    fragments: Vec<FragmentReport>,
    counts: Vec<EventCount>,
}

#[derive(Serialize)]
struct SourceReport {
    path: PathBuf,
    allocated_bytes: Option<u64>,
    same_inode_as: Option<PathBuf>,
}

#[derive(Serialize)]
struct FragmentReport {
    path: PathBuf,
    member: Option<String>,
    #[serde(flatten)]
    stats: FragmentStats,
}

#[derive(Serialize)]
struct EventCount {
    kind: Kind,
    skill: String,
    #[serde(flatten)]
    count: Count,
}

fn reference_paths(refs: &References) -> Result<BTreeMap<&str, Option<&str>>> {
    let mut edges = BTreeMap::<&str, Vec<&str>>::new();
    for [from, to] in &refs.edges {
        if !valid_id(from) || !valid_id(to) {
            return Err("invalid dependency identity".into());
        }
        edges.entry(from).or_default().push(to);
    }
    let mut parent = BTreeMap::new();
    let mut queue = VecDeque::new();
    for root in &refs.roots {
        if !valid_id(root) {
            return Err("invalid dependency root".into());
        }
        if parent.insert(root.as_str(), None).is_none() {
            queue.push_back(root.as_str());
        }
    }
    while let Some(from) = queue.pop_front() {
        for &to in edges.get(from).into_iter().flatten() {
            if let std::collections::btree_map::Entry::Vacant(entry) = parent.entry(to) {
                entry.insert(Some(from));
                queue.push_back(to);
            }
        }
    }
    Ok(parent)
}

fn cover(stats: &[FragmentStats]) -> &'static str {
    if stats.is_empty() || stats.iter().all(|stat| stat.status == "unreadable") {
        "unreadable"
    } else if stats.iter().all(|stat| stat.status == "unsupported") {
        "unsupported"
    } else if stats.iter().all(|stat| stat.status == "complete") {
        "complete"
    } else {
        "partial"
    }
}

fn snapshot(metadata: &Metadata) -> (u64, u64, u64, i64, i64, i64, i64) {
    (
        metadata.dev(),
        metadata.ino(),
        metadata.len(),
        metadata.mtime(),
        metadata.mtime_nsec(),
        metadata.ctime(),
        metadata.ctime_nsec(),
    )
}

pub(crate) fn scan(manifest: Manifest, manifest_path: &Path) -> Result<Report> {
    let limits = &manifest.limits;
    if manifest.schema != 1
        || limits.seconds == 0
        || limits.stored_bytes == 0
        || limits.decoded_bytes == 0
        || manifest.chats.is_empty()
        || manifest.chats.len() > 1024
        || manifest.skills.is_empty()
        || !manifest.skills.iter().all(|skill| valid_id(skill))
    {
        return Err("invalid archive schema, limits, population, or skills".into());
    }
    let paths = reference_paths(&manifest.references)?;
    let mut ids = BTreeSet::new();
    let mut sources = BTreeMap::<PathBuf, BTreeMap<Option<String>, Address>>::new();
    let parent = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let mut stats = Vec::new();
    for (chat_index, chat) in manifest.chats.iter().enumerate() {
        if !valid_id(&chat.id) || !ids.insert(&chat.id) {
            return Err("invalid or duplicate chat identity".into());
        }
        stats.push(
            chat.fragments
                .iter()
                .map(|_| FragmentStats::new("unreadable"))
                .collect::<Vec<_>>(),
        );
        for (fragment_index, fragment) in chat.fragments.iter().enumerate() {
            if fragment.path.as_os_str().is_empty()
                || fragment.member.as_ref().is_some_and(|member| {
                    member.is_empty()
                        || !Path::new(member)
                            .components()
                            .all(|part| matches!(part, Component::Normal(_)))
                })
            {
                return Err("invalid archive fragment path".into());
            }
            let target = sources.entry(parent.join(&fragment.path)).or_default();
            if target
                .insert(fragment.member.clone(), (chat_index, fragment_index))
                .is_some()
            {
                return Err("archive fragment appears more than once in manifest".into());
            }
            if target.len() > 1 && target.contains_key(&None) {
                return Err("source used both as plain file and archive".into());
            }
        }
    }

    let deadline = Instant::now()
        .checked_add(Duration::from_secs(limits.seconds))
        .ok_or("archive deadline is outside the supported clock range")?;
    let stored = Budget::default();
    let decoded = Budget::default();
    let mut scan = Scan {
        manifest: &manifest,
        deadline,
        stored: &stored,
        decoded: &decoded,
        event_budget: MAX_EVENTS,
        counts: (0..manifest.chats.len())
            .map(|_| Counts::default())
            .collect(),
    };
    let mut allocation = BTreeMap::<(u64, u64), (u64, &Path)>::new();
    let mut plain_copies = BTreeMap::new();
    let mut physical_sources = Vec::new();
    for (path, targets) in &sources {
        let mut physical = SourceReport {
            path: path.clone(),
            allocated_bytes: None,
            same_inode_as: None,
        };
        // Only regular named sources are opened, and allocation is charged
        // once per observed inode, even when it has multiple paths.
        let within_budget = scan.event_budget != 0
            && Instant::now() < deadline
            && scan.stored.stop.get().is_none()
            && scan.decoded.stop.get().is_none();
        if within_budget {
            if let Ok(file) = regular(path) {
                let before = file.metadata()?;
                let inode = (before.dev(), before.ino());
                let before_snapshot = snapshot(&before);
                let plain = targets
                    .get(&None)
                    .zip(codec(path))
                    .map(|(&(chat, index), codec)| ((before_snapshot, chat, codec), (chat, index)));
                let cached = plain
                    .as_ref()
                    .and_then(|(key, _)| plain_copies.get(key))
                    .cloned();
                let verification_handle = file.try_clone()?;
                let complete = match (plain.as_ref(), cached) {
                    (Some((_, (chat, index))), Some(cached)) => {
                        stats[*chat][*index] = cached;
                        true
                    }
                    _ => scan.source(path, file, targets, &mut stats),
                };
                let unchanged = snapshot(&verification_handle.metadata()?) == before_snapshot
                    && fs::symlink_metadata(path).is_ok_and(|current| {
                        current.is_file() && snapshot(&current) == before_snapshot
                    });
                if unchanged {
                    if let Some((_, first)) = allocation.get(&inode) {
                        physical.same_inode_as = Some(first.to_path_buf());
                    } else {
                        allocation.insert(inode, (before.blocks() * 512, path));
                    }
                    physical.allocated_bytes = Some(before.blocks() * 512);
                    if let Some((key, (chat, index))) = plain {
                        if complete && stats[chat][index].status == "complete" {
                            plain_copies.insert(key, stats[chat][index].clone());
                        }
                    }
                }
                if !complete || !unchanged {
                    for &(chat, fragment) in targets.values() {
                        if stats[chat][fragment].status == "complete" {
                            stats[chat][fragment].status = "partial";
                            stats[chat][fragment].issue = Some(if unchanged {
                                "archive container coverage is incomplete"
                            } else {
                                "source changed during observation"
                            });
                        }
                    }
                }
            }
        }
        physical_sources.push(physical);
    }

    let mut report = Report {
        registered_chats: manifest.chats.len(),
        complete_chats: 0,
        partial_chats: 0,
        unreadable_chats: 0,
        unsupported_chats: 0,
        native_load_eligible_chats: 0,
        stored_bytes_read: scan.stored.used.get(),
        decoded_bytes_processed: scan.decoded.used.get(),
        allocated_bytes_observed: allocation.values().map(|(bytes, _)| bytes).sum(),
        sources_with_unknown_allocation: physical_sources
            .iter()
            .filter(|source| source.allocated_bytes.is_none())
            .count(),
        sources: physical_sources,
        stop_reason: scan
            .stored
            .stop
            .get()
            .or(scan.decoded.stop.get())
            .or_else(|| (scan.event_budget == 0).then_some("event_limit"))
            .or_else(|| (Instant::now() >= deadline).then_some("time_limit")),
        chats: Vec::new(),
    };
    for ((chat, stats), counts) in manifest.chats.iter().zip(stats).zip(scan.counts) {
        let status = cover(&stats);
        let native_load_eligible =
            status == "complete" && chat.fragments.iter().all(|fragment| fragment.native_events);
        report.native_load_eligible_chats += usize::from(native_load_eligible);
        match status {
            "complete" => report.complete_chats += 1,
            "partial" => report.partial_chats += 1,
            "unreadable" => report.unreadable_chats += 1,
            _ => report.unsupported_chats += 1,
        }
        let mut path = Vec::new();
        let mut current = chat.id.as_str();
        let mut reference_path_truncated = false;
        while let Some(parent) = paths.get(current) {
            if path.len() == MAX_REFERENCE_PATH {
                reference_path_truncated = true;
                break;
            }
            path.push(current.to_string());
            match parent {
                Some(next) => current = next,
                None => break,
            }
        }
        path.reverse();
        let retention_reference = if !path.is_empty() {
            "protected_by_reference"
        } else if manifest.references.complete {
            "unreferenced_in_snapshot"
        } else {
            "unknown"
        };
        let fragments = chat
            .fragments
            .iter()
            .zip(stats)
            .map(|(fragment, stat)| FragmentReport {
                path: fragment.path.clone(),
                member: fragment.member.clone(),
                stats: stat,
            })
            .collect();
        let counts = counts
            .buckets
            .into_iter()
            .map(|((kind, skill), count)| EventCount { kind, skill, count })
            .collect();
        report.chats.push(ChatReport {
            id: chat.id.clone(),
            status,
            native_load_eligible,
            retention_reference,
            reference_path: path,
            reference_path_truncated,
            fragments,
            counts,
        });
    }
    Ok(report)
}
