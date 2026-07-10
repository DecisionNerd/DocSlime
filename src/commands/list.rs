//! `docslime list` — show available templates and whether each exists on disk.

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
        let legacy = templates::legacy_paths(rel)
            .iter()
            .map(|relative| scaffold::output_path(root, Path::new(relative)))
            .find(|path| path.exists());
        let status = if on_disk {
            "exists ".green().to_string()
        } else if legacy.is_some() {
            "legacy ".yellow().to_string()
        } else {
            "missing".dimmed().to_string()
        };
        if let Some(legacy) = legacy.filter(|_| !on_disk) {
            println!("  {status}  {} ({})", rel.display(), legacy.display());
        } else {
            println!("  {status}  {}", rel.display());
        }
    }

    println!(
        "\n  {}  create the next-numbered ADR via `docslime add adr <slug>`",
        "adr".cyan()
    );
    Ok(())
}
