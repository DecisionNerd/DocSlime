//! The single filesystem write chokepoint.
//!
//! Every file the tool emits goes through [`write_file`], which owns the skip / overwrite
//! policy and the mapping from a template's relative path to its location under `docs/`.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

/// The directory, relative to the target root, that all templates are written into.
pub const DOCS_DIR: &str = "docs";

/// What happened when a single file was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Created,
    Skipped,
    Overwritten,
}

/// Running tally of write outcomes, for end-of-command summaries.
#[derive(Debug, Default, Clone, Copy)]
pub struct Summary {
    pub created: usize,
    pub skipped: usize,
    pub overwritten: usize,
}

impl Summary {
    pub fn record(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Created => self.created += 1,
            Outcome::Skipped => self.skipped += 1,
            Outcome::Overwritten => self.overwritten += 1,
        }
    }
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} created, {} skipped, {} overwritten",
            self.created, self.skipped, self.overwritten
        )
    }
}

/// Map a template's relative path to its output path under `<root>/docs/`.
pub fn output_path(root: &Path, relative: &Path) -> PathBuf {
    root.join(DOCS_DIR).join(relative)
}

/// Write `contents` to `path`, honoring the skip / overwrite policy.
///
/// This is the *only* place files are created. Parent directories are created as needed.
/// Existing files are left untouched unless `force` is set. Each call prints one status line.
pub fn write_file(path: &Path, contents: &[u8], force: bool) -> Result<Outcome> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    let exists = path.exists();
    if exists && !force {
        report(Outcome::Skipped, path);
        return Ok(Outcome::Skipped);
    }

    fs::write(path, contents)
        .with_context(|| format!("failed to write {}", path.display()))?;

    let outcome = if exists {
        Outcome::Overwritten
    } else {
        Outcome::Created
    };
    report(outcome, path);
    Ok(outcome)
}

/// Print a single colored status line for a write outcome.
fn report(outcome: Outcome, path: &Path) {
    let p = path.display();
    match outcome {
        Outcome::Created => println!("  {}  {p}", "create".green()),
        Outcome::Skipped => println!("  {}    {p}", "skip".dimmed()),
        Outcome::Overwritten => println!("  {} {p}", "overwrite".yellow()),
    }
}
