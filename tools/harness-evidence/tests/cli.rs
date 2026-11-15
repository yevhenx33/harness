use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::Cursor;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "harness-evidence-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn json(&self, name: &str, value: &Value) {
        fs::write(self.0.join(name), serde_json::to_vec(value).unwrap()).unwrap();
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_harness-evidence"))
            .current_dir(&self.0)
            .args(args)
            .output()
            .unwrap()
    }

    fn report(&self, args: &[&str]) -> Value {
        let result = self.run(args);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        serde_json::from_slice(&result.stdout).unwrap()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn independent_named_expectations_decide_the_exact_execution() {
    let scratch = Scratch::new();
    let identity = json!({
        "candidate": "source+bundle", "package": "server", "target": "bin:server",
        "configuration": "release+fixtures", "executable_sha256": "1".repeat(64),
        "run": "run-123:attempt-1"
    });
    scratch.json("plan.json", &json!({
        "schema": 1,
        "expected": identity,
        "guarantees": [
            {"id":"published", "owner":"publication", "source":"src/publication.rs::commit",
             "outcome":"the new writer commits", "cases":["stale_refused","replacement_commits"]},
            {"id":"fencing", "owner":"publication", "source":"src/publication.rs::fence",
             "outcome":"old writers are refused", "cases":["stale_refused"]}
        ]
    }));
    let observed = json!({
        "schema": 1, "execution": identity, "outcome":"passed",
        "cases":[{"id":"stale_refused","outcome":"passed"},{"id":"replacement_commits","outcome":"passed"}]
    });
    let assess = |record: &Value| {
        scratch.json("observed.json", record);
        scratch.report(&["check-run", "plan.json", "observed.json"])
    };
    assert_eq!(assess(&observed)["status"], "verified");

    for (field, wrong) in [
        ("target", json!("lib:sdk")),
        ("package", json!("sdk")),
        ("run", json!("run-123:attempt-0")),
        ("candidate", json!("previous-bundle")),
        ("configuration", json!("debug")),
        ("executable_sha256", json!("2".repeat(64))),
    ] {
        let mut wrong_record = observed.clone();
        wrong_record["execution"][field] = wrong;
        let result = assess(&wrong_record);
        assert_eq!(result["status"], "unverified");
        assert_eq!(result["mismatched_fields"], json!([field]));
        assert_eq!(result["guarantees"][0]["missing"], json!([]));
    }

    let mut missing = observed.clone();
    missing["cases"].as_array_mut().unwrap().pop();
    let report = assess(&missing);
    assert_eq!(report["status"], "unverified");
    assert_eq!(
        report["guarantees"][0]["missing"],
        json!(["replacement_commits"])
    );
    assert_eq!(report["guarantees"][1]["status"], "verified");

    let mut skipped = observed.clone();
    skipped["cases"][1]["outcome"] = json!("skipped");
    let report = assess(&skipped);
    assert_eq!(report["status"], "unverified");
    assert_eq!(
        report["guarantees"][0]["skipped"],
        json!(["replacement_commits"])
    );

    let mut incomplete = observed.clone();
    incomplete["outcome"] = json!("incomplete");
    incomplete["cases"][1]["outcome"] = json!("incomplete");
    let report = assess(&incomplete);
    assert_eq!(report["status"], "unverified");
    assert_eq!(
        report["guarantees"][0]["incomplete"],
        json!(["replacement_commits"])
    );
    assert_eq!(report["guarantees"][0]["missing"], json!([]));

    let mut failed = observed.clone();
    failed["outcome"] = json!("failed");
    failed["cases"][1]["outcome"] = json!("failed");
    let report = assess(&failed);
    assert_eq!(report["status"], "failed");
    assert_eq!(
        report["guarantees"][0]["failed"],
        json!(["replacement_commits"])
    );

    let mut contradictory = observed.clone();
    contradictory["cases"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"other","outcome":"failed"}));
    scratch.json("observed.json", &contradictory);
    assert_eq!(
        scratch
            .run(&["check-run", "plan.json", "observed.json"])
            .status
            .code(),
        Some(2)
    );

    let mut duplicate = observed.clone();
    duplicate["cases"]
        .as_array_mut()
        .unwrap()
        .push(observed["cases"][0].clone());
    scratch.json("observed.json", &duplicate);
    assert_eq!(
        scratch
            .run(&["check-run", "plan.json", "observed.json"])
            .status
            .code(),
        Some(2)
    );
}

fn jsonl(rows: &[Value]) -> Vec<u8> {
    rows.iter()
        .flat_map(|row| [serde_json::to_vec(row).unwrap(), vec![b'\n']].concat())
        .collect()
}

fn meta(id: &str) -> Value {
    json!({"type":"session_meta", "payload":{"id":id}})
}

fn native() -> Value {
    json!({"type":"response_item", "payload":{
        "type":"message", "role":"user", "id":"injection-1",
        "content":[{"type":"input_text","text":"<skill>\n<name>clean</name>\n<path>/skills/clean/SKILL.md</path>\n$clean\n</skill>"}],
        "internal_chat_message_metadata_passthrough":{
            "turn_id":"turn-1", "content_item_kinds":["skills.selected_skill_instructions"],
            "content_item_metadata":[{"provenance":{"type":"skills"}}]
        }
    }})
}

fn add_member<W: std::io::Write>(tar: &mut tar::Builder<W>, path: &str, data: &[u8]) {
    let mut header = tar::Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_mode(0o600);
    header.set_cksum();
    tar.append_data(&mut header, path, Cursor::new(data))
        .unwrap();
}

fn chat<'a>(report: &'a Value, id: &str) -> &'a Value {
    report["chats"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == id)
        .unwrap()
}

fn event<'a>(chat: &'a Value, kind: &str) -> &'a Value {
    chat["counts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["kind"] == kind)
        .unwrap()
}

#[test]
fn archive_preserves_provenance_coverage_and_recovery_roots() {
    let scratch = Scratch::new();
    let call = |id| {
        json!({"type":"response_item","payload":{
            "type":"function_call","call_id":id,"name":"exec_command",
            "arguments":"{\"cmd\":\"echo /skills/clean/SKILL.md\"}"
        }})
    };
    let plain = jsonl(&[
        meta("parent"),
        json!({"type":"response_item","payload":{"type":"message","id":"request-1","role":"user",
            "content":[{"type":"input_text","text":"Please run $clean"}],
            "internal_chat_message_metadata_passthrough":{"turn_id":"turn-1"}
        }}),
        call("call-1"),
        call("call-2"),
        native(),
        json!({"type":"response_item","payload":{"type":"function_call_output","output":"private-output-body"}}),
    ]);
    fs::write(scratch.0.join("first.jsonl"), &plain).unwrap();
    fs::hard_link(scratch.0.join("first.jsonl"), scratch.0.join("copy.jsonl")).unwrap();
    let mut partial = jsonl(&[meta("parent"), native()]);
    partial.extend_from_slice(b"{\"type\":");
    let long_member = format!("nested/{}/fragment.jsonl.zst", "x".repeat(120));
    let out = File::create(scratch.0.join("archive.tar.zst")).unwrap();
    let mut tar = tar::Builder::new(zstd::stream::write::Encoder::new(out, 1).unwrap());
    add_member(
        &mut tar,
        &long_member,
        &zstd::encode_all(Cursor::new(partial), 1).unwrap(),
    );
    add_member(&mut tar, "complete.jsonl", &jsonl(&[meta("second")]));
    let mut symlink = tar::Header::new_gnu();
    symlink.set_entry_type(tar::EntryType::Symlink);
    symlink.set_size(0);
    tar.append_link(&mut symlink, "link.jsonl", "complete.jsonl")
        .unwrap();
    tar.into_inner().unwrap().finish().unwrap();
    fs::write(scratch.0.join("unknown.data"), b"not a transcript").unwrap();

    let manifest = json!({
        "schema":1,"limits":{"seconds":10,"stored_bytes":1000000,"decoded_bytes":1000000},
        "skills":["clean"],
        "references":{"complete":false, "roots":["active-fork","backup-current"],
            "edges":[["active-fork","intermediate"],["intermediate","parent"],["backup-current","second"],["parent","intermediate"]]},
        "chats":[
            {"id":"parent","fragments":[
                {"path":"first.jsonl","native_events":true},{"path":"copy.jsonl","native_events":true},
                {"path":"archive.tar.zst","member":long_member,"native_events":true}
            ]},
            {"id":"second","fragments":[{"path":"archive.tar.zst","member":"complete.jsonl","native_events":true}]},
            {"id":"linked","fragments":[{"path":"archive.tar.zst","member":"link.jsonl","native_events":true}]},
            {"id":"unknown","fragments":[{"path":"unknown.data","native_events":false}]},
            {"id":"missing","fragments":[{"path":"absent.jsonl","native_events":true}]}
        ]
    });
    scratch.json("manifest.json", &manifest);
    let output = scratch.report(&["archive", "manifest.json"]);
    assert!(!output.to_string().contains("private-output-body"));
    assert_eq!(output["registered_chats"], 5);
    assert_eq!(output["complete_chats"], 1);
    assert_eq!(output["partial_chats"], 1);
    assert_eq!(output["unreadable_chats"], 1);
    assert_eq!(output["unsupported_chats"], 2);
    assert_eq!(output["native_load_eligible_chats"], 1);
    let parent = chat(&output, "parent");
    assert_eq!(parent["status"], "partial");
    assert_eq!(parent["retention_reference"], "protected_by_reference");
    assert_eq!(
        parent["reference_path"],
        json!(["active-fork", "intermediate", "parent"])
    );
    assert_eq!(event(parent, "native_load")["identified_events"], 1);
    assert_eq!(event(parent, "request")["identified_events"], 1);
    assert_eq!(event(parent, "path_reference")["identified_events"], 2);
    assert_eq!(event(parent, "path_reference")["known_turns"], 0);
    assert_eq!(
        event(parent, "path_reference")["observations_without_turn"],
        2
    );
    assert!(
        parent["fragments"][2]["transcript_bytes_observed"]
            .as_u64()
            .unwrap()
            > 0
    );
    assert_eq!(
        chat(&output, "second")["reference_path"],
        json!(["backup-current", "second"])
    );
    assert_eq!(chat(&output, "unknown")["retention_reference"], "unknown");
    let allocated = ["first.jsonl", "archive.tar.zst", "unknown.data"]
        .iter()
        .map(|path| fs::metadata(scratch.0.join(path)).unwrap().blocks() * 512)
        .sum::<u64>();
    assert_eq!(output["allocated_bytes_observed"], allocated);

    let mut bounded = manifest;
    bounded["limits"]["stored_bytes"] = json!(5);
    scratch.json("manifest.json", &bounded);
    let result = scratch.report(&["archive", "manifest.json"]);
    assert_eq!(result["stop_reason"], "stored_byte_limit");
    assert_eq!(result["stored_bytes_read"], 5);
    bounded["limits"]["stored_bytes"] = json!(1000000);
    bounded["limits"]["decoded_bytes"] = json!(31);
    scratch.json("manifest.json", &bounded);
    let result = scratch.report(&["archive", "manifest.json"]);
    assert_eq!(result["stop_reason"], "decoded_byte_limit");
    assert!(result["decoded_bytes_processed"].as_u64().unwrap() <= 31);
}

#[test]
fn archive_marks_incomplete_framing_and_contradictory_metadata() {
    let scratch = Scratch::new();
    let data = jsonl(&[meta("archived"), native()]);
    let mut torn = tar::Builder::new(Vec::new());
    add_member(&mut torn, "first.jsonl", &data);
    let mut torn = torn.into_inner().unwrap();
    torn.truncate(torn.len() - 512);
    fs::write(scratch.0.join("torn.tar"), torn).unwrap();
    let mut contradictory = native();
    contradictory["payload"]["internal_chat_message_metadata_passthrough"]["turn_id"] =
        json!("wrong");
    fs::write(
        scratch.0.join("z-second.jsonl"),
        jsonl(&[meta("archived"), contradictory]),
    )
    .unwrap();
    let mut manifest = json!({
        "schema":1,"limits":{"seconds":10,"stored_bytes":1000000,"decoded_bytes":1000000},
        "skills":["clean"],"references":{"complete":true,"roots":[],"edges":[]},
        "chats":[{"id":"archived","fragments":[
            {"path":"torn.tar","member":"first.jsonl","native_events":true},
            {"path":"z-second.jsonl","native_events":true}
        ]}]
    });
    scratch.json("manifest.json", &manifest);
    let report = scratch.report(&["archive", "manifest.json"]);
    assert_eq!(report["partial_chats"], 1);
    let archived = chat(&report, "archived");
    assert_eq!(archived["fragments"][0]["status"], "partial");
    assert_eq!(archived["fragments"][1]["status"], "partial");
    assert_eq!(archived["native_load_eligible"], false);
    assert_eq!(event(archived, "native_load")["known_turns"], 0);
    assert_eq!(
        event(archived, "native_load")["observations_without_turn"],
        1
    );
    assert_eq!(
        archived["fragments"][1]["issue"],
        "conflicting turn for copied event"
    );

    manifest["chats"][0]["fragments"][0]["member"] = json!("../first.jsonl");
    scratch.json("manifest.json", &manifest);
    let rejected = scratch.run(&["archive", "manifest.json"]);
    assert_eq!(rejected.status.code(), Some(2));
    assert!(rejected.stdout.is_empty());
}

#[test]
fn refuses_symlinked_evidence_file() {
    let scratch = Scratch::new();
    let hidden = scratch.0.join("evidence.json");
    fs::write(&hidden, b"{}").unwrap();
    std::os::unix::fs::symlink(Path::new("evidence.json"), scratch.0.join("link.json")).unwrap();
    assert_eq!(
        scratch.run(&["archive", "link.json"]).status.code(),
        Some(2)
    );
}

#[test]
fn archive_counts_structured_arguments_and_surfaces_invalid_evidence() {
    let scratch = Scratch::new();
    let rows = [
        meta("session"),
        json!({"type":"response_item","payload":{
            "type":"message","role":"developer","id":"catalog",
            "content":[{"type":"input_text","text":"Available skill: $clean"}],
            "internal_chat_message_metadata_passthrough":{
                "content_item_kinds":["host_skills.instructions"],
                "content_item_metadata":[{"provenance":{"type":"skills"}}]
            }
        }}),
        json!({"type":"response_item","payload":{
            "type":"function_call", "call_id":"call-1","arguments":{"steps":[{"path":"/workspace/skills/clean/SKILL.md"}]},
            "turn_id":"turn-1"
        }}),
        json!({"type":"response_item","payload":{
            "type":"custom_tool_call", "call_id":"call-2","input":null
        }}),
    ];
    fs::write(scratch.0.join("trace.jsonl"), jsonl(&rows)).unwrap();
    scratch.json(
        "manifest.json",
        &json!({
            "schema":1,"limits":{"seconds":10,"stored_bytes":1000000,"decoded_bytes":1000000},
            "skills":["clean"],"references":{"complete":false,"roots":[],"edges":[]},
            "chats":[{"id":"session","fragments":[{"path":"trace.jsonl","native_events":true}]}]
        }),
    );
    let report = scratch.report(&["archive", "manifest.json"]);
    let session = chat(&report, "session");
    assert_eq!(session["status"], "partial");
    assert_eq!(session["native_load_eligible"], false);
    assert_eq!(
        session["fragments"][0]["issue"],
        "invalid tool call arguments"
    );
    assert_eq!(event(session, "path_reference")["known_turns"], 1);
    assert_eq!(event(session, "path_reference")["identified_events"], 1);
}

#[test]
fn archive_excludes_a_native_slot_with_conflicting_skill_names() {
    let scratch = Scratch::new();
    let mut conflicting = native();
    conflicting["payload"]["content"][0]["text"] = json!("<skill>\n<name>map</name>\n</skill>");
    fs::write(
        scratch.0.join("trace.jsonl"),
        jsonl(&[meta("session"), native(), conflicting]),
    )
    .unwrap();
    scratch.json(
        "manifest.json",
        &json!({
            "schema":1,"limits":{"seconds":10,"stored_bytes":1000000,"decoded_bytes":1000000},
            "skills":["clean"],"references":{"complete":false,"roots":[],"edges":[]},
            "chats":[{"id":"session","fragments":[{"path":"trace.jsonl","native_events":true}]}]
        }),
    );
    let report = scratch.report(&["archive", "manifest.json"]);
    let session = chat(&report, "session");
    assert_eq!(session["status"], "partial");
    assert_eq!(
        session["fragments"][0]["issue"],
        "conflicting skill for copied native event"
    );
    let load = event(session, "native_load");
    assert_eq!(load["identified_events"], 0);
    assert_eq!(load["conflicting_identities"], 1);
    assert_eq!(load["known_turns"], 0);
}

#[test]
fn lifecycle_witness_requires_the_replacement_to_commit() {
    let scratch = Scratch::new();
    let identity = json!({
        "candidate":"synthetic-lifecycle", "package":"fixture", "target":"bin:publisher",
        "configuration":"durable-file", "executable_sha256":"1".repeat(64), "run":"interrupt-and-resume"
    });
    scratch.json("plan.json", &json!({
        "schema":1,"expected":identity,
        "guarantees":[{
            "id":"takeover","owner":"fixture publication writer","source":"tests/cli.rs::lifecycle_witness_requires_the_replacement_to_commit",
            "outcome":"the replacement commits and the old writer cannot change the durable record",
            "cases":["replacement_commits","stale_write_refused"]
        }]
    }));
    let state = scratch.0.join("publication");
    let current = scratch.0.join("epoch");
    let temp = scratch.0.join("pending");
    for (mode, expected_failure) in [
        ("fenced", None),
        ("allow-stale", Some("stale_write_refused")),
        ("refuse-all", Some("replacement_commits")),
    ] {
        fs::write(&current, "1").unwrap();
        fs::write(&state, "1|initial").unwrap();
        let paused_writer = fs::read_to_string(&current).unwrap();
        let commit = |epoch: &str, name: &str| {
            if mode != "refuse-all"
                && (mode == "allow-stale" || fs::read_to_string(&current).unwrap() == epoch)
            {
                fs::write(&temp, format!("{epoch}|{name}")).unwrap();
                fs::rename(&temp, &state).unwrap();
            }
        };

        // Fence the admission, let a replacement publish, then resume the
        // prior attempt. The reader observes disk independently of commit.
        fs::write(&current, "2").unwrap();
        commit("2", "replacement");
        let before_resume = fs::read_to_string(&state).unwrap();
        commit(&paused_writer, "stale");
        let after_resume = fs::read_to_string(&state).unwrap();
        let progressed = before_resume == "2|replacement";
        let stale_refused = after_resume == before_resume;
        let result = |passed| if passed { "passed" } else { "failed" };
        scratch.json(
            "observed.json",
            &json!({
                "schema":1,"execution":identity,"outcome":result(progressed && stale_refused),
                "cases":[{"id":"replacement_commits","outcome":result(progressed)},
                         {"id":"stale_write_refused","outcome":result(stale_refused)}]
            }),
        );
        let report = scratch.report(&["check-run", "plan.json", "observed.json"]);
        match expected_failure {
            Some(case) => {
                assert_eq!(report["status"], "failed", "{mode}");
                assert_eq!(report["guarantees"][0]["failed"], json!([case]), "{mode}");
            }
            None => assert_eq!(report["status"], "verified", "{mode}"),
        }
    }
}
