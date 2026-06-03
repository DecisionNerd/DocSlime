//! `docgen list` — show available templates and whether each exists on disk.

use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::scaffold;
use crate::templates;

/// Print every template path with an `exists`/`missing` status relative to `<root>/docs/`.
pub fn run(root: &Path) -> Result<()> {
    println!("Templates (relative to {}/):\n", scaffold::DOCS_DIR.bold());

    for file in templates::all() {
        let rel = file.path();
        let on_disk = scaffold::output_path(root, rel).exists();
        let status = if on_disk {
            "exists ".green().to_string()
        } else {
            "missing".dimmed().to_string()
        };
        println!("  {status}  {}", rel.display());
    }

    println!(
        "\n  {}  create the next-numbered ADR via `docgen add adr <slug>`",
        "adr".cyan()
    );
    Ok(())
}
