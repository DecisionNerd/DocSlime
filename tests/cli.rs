//! Black-box integration tests for the `docgen` CLI.

use std::fs;
use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// The 12 files `init` is expected to create, relative to `docs/`.
const EXPECTED: &[&str] = &[
    "README.md",
    "0-MISSION.md",
    "1-EXPERIENCES.md",
    "2-REQUIREMENTS.md",
    "3-ARCHITECTURE.md",
    "4-TESTING.md",
    "0-PRODUCT/README.md",
    "1-JOURNEYS/README.md",
    "2-DESIGN/README.md",
    "2-DESIGN/style-guide.md",
    "3-ENGINEERING/README.md",
    "3-ENGINEERING/ADRs/README.md",
];

fn docgen(dir: &Path) -> Command {
    let mut cmd = Command::cargo_bin("docgen").unwrap();
    cmd.current_dir(dir);
    cmd
}

#[test]
fn init_creates_full_tree() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path()).arg("init").assert().success();

    for rel in EXPECTED {
        let path = tmp.path().join("docs").join(rel);
        assert!(path.is_file(), "expected {} to exist", path.display());
    }
}

#[test]
fn every_template_carries_llm_guidance() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path()).arg("init").assert().success();

    for rel in EXPECTED {
        let contents = fs::read_to_string(tmp.path().join("docs").join(rel)).unwrap();
        assert!(
            contents.contains("<!-- LLM:"),
            "{rel} is missing LLM guidance comments"
        );
    }
}

#[test]
fn init_skips_existing_files() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path()).arg("init").assert().success();

    // Mark one file, re-run init, and confirm it was left untouched.
    let mission = tmp.path().join("docs/0-MISSION.md");
    fs::write(&mission, "USER EDITS").unwrap();

    docgen(tmp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("skipped"));

    assert_eq!(fs::read_to_string(&mission).unwrap(), "USER EDITS");
}

#[test]
fn init_force_overwrites() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path()).arg("init").assert().success();

    let mission = tmp.path().join("docs/0-MISSION.md");
    fs::write(&mission, "GARBAGE").unwrap();

    docgen(tmp.path())
        .args(["init", "--force"])
        .assert()
        .success();

    let restored = fs::read_to_string(&mission).unwrap();
    assert_ne!(restored, "GARBAGE");
    assert!(restored.contains("# Mission"));
}

#[test]
fn add_resolves_shorthand_name() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path())
        .args(["add", "3-ARCHITECTURE"])
        .assert()
        .success();

    assert!(tmp.path().join("docs/3-ARCHITECTURE.md").is_file());
}

#[test]
fn add_unknown_template_fails() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path())
        .args(["add", "bogus"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no template matches"));
}

#[test]
fn add_adr_numbers_sequentially() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path()).arg("init").assert().success();

    docgen(tmp.path())
        .args(["add", "adr", "My First Decision!"])
        .assert()
        .success();
    docgen(tmp.path())
        .args(["add", "adr", "second-one"])
        .assert()
        .success();

    let adr_dir = tmp.path().join("docs/3-ENGINEERING/ADRs");
    assert!(adr_dir.join("0001-my-first-decision.md").is_file());
    assert!(adr_dir.join("0002-second-one.md").is_file());
}

#[test]
fn add_adr_starts_at_one_without_init() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path())
        .args(["add", "adr", "first"])
        .assert()
        .success();

    assert!(tmp
        .path()
        .join("docs/3-ENGINEERING/ADRs/0001-first.md")
        .is_file());
}

#[test]
fn add_adr_requires_slug() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path())
        .args(["add", "adr"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("requires a slug"));
}

#[test]
fn list_shows_every_template() {
    let tmp = TempDir::new().unwrap();
    let assert = docgen(tmp.path()).arg("list").assert().success();
    let out = assert.get_output();
    let stdout = String::from_utf8_lossy(&out.stdout);

    for rel in EXPECTED {
        assert!(stdout.contains(rel), "list output missing {rel}");
    }
}

#[test]
fn list_reflects_on_disk_status() {
    let tmp = TempDir::new().unwrap();
    docgen(tmp.path()).arg("init").assert().success();

    docgen(tmp.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("exists"));
}
