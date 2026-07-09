//! `docslime init` — scaffold the entire docs/ tree.

use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::scaffold::{self, Summary};
use crate::templates;

/// Write every embedded template into `<root>/docs/`, then print a summary.
pub fn run(root: &Path, force: bool) -> Result<()> {
    let mut summary = Summary::default();

    for file in templates::all() {
        let dest = scaffold::output_path(root, file.path());
        let outcome = scaffold::write_file(&dest, file.contents(), force)?;
        summary.record(outcome);
    }

    println!("\n{} {summary}", "docs/".bold());
    Ok(())
}
