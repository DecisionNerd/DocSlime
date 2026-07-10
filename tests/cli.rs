//! Black-box integration tests for the `docslime` CLI.

use std::fs;
use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// The 12 files `init` is expected to create, relative to `docs/`.
const EXPECTED: &[&str] = &[
    "README.md",
    "PRODUCT.md",
    "DESIGN.md",
    "REQUIREMENTS.md",
    "strategy/README.md",
    "experience/README.md",
    "engineering/README.md",
    "engineering/ARCHITECTURE.md",
    "engineering/TESTING.md",
    "engineering/PUBLISHING.md",
    "engineering/OBSERVABILITY.md",
    "engineering/adrs/README.md",
];

fn docslime(dir: &Path) -> Command {
    let mut cmd = Command::cargo_bin("docslime").unwrap();
    cmd.current_dir(dir);
    cmd
}

#[test]
fn init_creates_full_tree() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path()).arg("init").assert().success();

    for rel in EXPECTED {
        let path = tmp.path().join("docs").join(rel);
        assert!(path.is_file(), "expected {} to exist", path.display());
    }
}

#[test]
fn every_template_carries_llm_guidance() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path()).arg("init").assert().success();

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
    docslime(tmp.path()).arg("init").assert().success();

    // Mark one file, re-run init, and confirm it was left untouched.
    let product = tmp.path().join("docs/PRODUCT.md");
    fs::write(&product, "USER EDITS").unwrap();

    docslime(tmp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("skipped"));

    assert_eq!(fs::read_to_string(&product).unwrap(), "USER EDITS");
}

#[test]
fn init_force_overwrites() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path()).arg("init").assert().success();

    let product = tmp.path().join("docs/PRODUCT.md");
    fs::write(&product, "GARBAGE").unwrap();

    docslime(tmp.path())
        .args(["init", "--force"])
        .assert()
        .success();

    let restored = fs::read_to_string(&product).unwrap();
    assert_ne!(restored, "GARBAGE");
    assert!(restored.contains("# Product"));
}

#[test]
fn add_resolves_shorthand_name() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path())
        .args(["add", "ARCHITECTURE"])
        .assert()
        .success();

    assert!(tmp
        .path()
        .join("docs/engineering/ARCHITECTURE.md")
        .is_file());
}

#[test]
fn add_resolves_legacy_shorthand_name() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path())
        .args(["add", "3-ARCHITECTURE"])
        .assert()
        .success();

    assert!(tmp
        .path()
        .join("docs/engineering/ARCHITECTURE.md")
        .is_file());
}

#[test]
fn init_recognizes_legacy_paths() {
    let tmp = TempDir::new().unwrap();
    let legacy = tmp.path().join("docs/3-ARCHITECTURE.md");
    fs::create_dir_all(legacy.parent().unwrap()).unwrap();
    fs::write(&legacy, "USER ARCHITECTURE").unwrap();

    docslime(tmp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("legacy:"));

    assert_eq!(fs::read_to_string(&legacy).unwrap(), "USER ARCHITECTURE");
    assert!(!tmp.path().join("docs/engineering/ARCHITECTURE.md").exists());
}

#[test]
fn list_recognizes_legacy_paths() {
    let tmp = TempDir::new().unwrap();
    let legacy = tmp.path().join("docs/4-TESTING.md");
    fs::create_dir_all(legacy.parent().unwrap()).unwrap();
    fs::write(legacy, "USER TESTING").unwrap();

    docslime(tmp.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("legacy"))
        .stdout(predicate::str::contains("engineering/TESTING.md"))
        .stdout(predicate::str::contains("4-TESTING.md"));
}

#[test]
fn add_unknown_template_fails() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path())
        .args(["add", "bogus"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no template matches"));
}

#[test]
fn add_adr_numbers_sequentially() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path()).arg("init").assert().success();

    docslime(tmp.path())
        .args(["add", "adr", "My First Decision!"])
        .assert()
        .success();
    docslime(tmp.path())
        .args(["add", "adr", "second-one"])
        .assert()
        .success();

    let adr_dir = tmp.path().join("docs/engineering/adrs");
    assert!(adr_dir.join("0001-my-first-decision.md").is_file());
    assert!(adr_dir.join("0002-second-one.md").is_file());
}

#[test]
fn add_adr_starts_at_one_without_init() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path())
        .args(["add", "adr", "first"])
        .assert()
        .success();

    assert!(tmp
        .path()
        .join("docs/engineering/adrs/0001-first.md")
        .is_file());
}

#[test]
fn add_adr_continues_in_legacy_directory() {
    let tmp = TempDir::new().unwrap();
    let legacy_dir = tmp.path().join("docs/3-ENGINEERING/ADRs");
    fs::create_dir_all(&legacy_dir).unwrap();
    fs::write(legacy_dir.join("0004-existing.md"), "EXISTING").unwrap();

    docslime(tmp.path())
        .args(["add", "adr", "next"])
        .assert()
        .success();

    assert!(legacy_dir.join("0005-next.md").is_file());
    assert!(!tmp.path().join("docs/engineering/adrs").exists());
}

#[test]
fn add_adr_requires_slug() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path())
        .args(["add", "adr"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("requires a slug"));
}

#[test]
fn list_shows_every_template() {
    let tmp = TempDir::new().unwrap();
    let assert = docslime(tmp.path()).arg("list").assert().success();
    let out = assert.get_output();
    let stdout = String::from_utf8_lossy(&out.stdout);

    for rel in EXPECTED {
        assert!(stdout.contains(rel), "list output missing {rel}");
    }
}

#[test]
fn list_reflects_on_disk_status() {
    let tmp = TempDir::new().unwrap();
    docslime(tmp.path()).arg("init").assert().success();

    docslime(tmp.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("exists"));
}

#[test]
fn kiss_is_not_a_cli_subcommand() {
    let tmp = TempDir::new().unwrap();

    docslime(tmp.path())
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("kiss").not());
}

#[test]
fn agent_skills_have_required_sections() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    let skills_dir = repo.join(".agents/skills");
    let required = [
        "docslime-install",
        "docslime-init",
        "docslime-fill",
        "docslime-adr",
        "docslime-kiss",
    ];

    for skill in required {
        let path = skills_dir.join(skill).join("SKILL.md");
        let contents = fs::read_to_string(&path).unwrap_or_else(|err| {
            panic!("failed to read {}: {err}", path.display());
        });

        assert!(contents.starts_with("---\n"), "{skill} missing frontmatter");
        assert!(
            contents.contains("name: ") && contents.contains("description: "),
            "{skill} missing required frontmatter fields"
        );

        for section in [
            "## When to Use",
            "## Prerequisites",
            "## Guardrails",
            "## Verification",
            "## Failure Handling",
        ] {
            assert!(
                contents.contains(section),
                "{skill} missing required section {section}"
            );
        }

        let metadata = skills_dir.join(skill).join("agents/openai.yaml");
        assert!(metadata.is_file(), "{skill} missing agents/openai.yaml");
    }
}
