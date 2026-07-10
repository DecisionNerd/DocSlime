//! `docslime add <doc>` — add a single document, or create the next-numbered ADR.

use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::scaffold;
use crate::templates::{self, FindError};

/// Relative location of the ADR directory within the docs tree.
const ADR_DIR: &str = "engineering/adrs";
const LEGACY_ADR_DIR: &str = "3-ENGINEERING/ADRs";

/// Dispatch `add`: the literal `adr` triggers numbered-ADR creation; anything else resolves
/// to a single embedded template.
pub fn run(root: &Path, doc: &str, slug: Option<&str>, force: bool) -> Result<()> {
    if doc.eq_ignore_ascii_case("adr") {
        return add_adr(root, slug, force);
    }

    if slug.is_some() {
        bail!("the second argument (slug) is only used with `add adr <slug>`");
    }

    match templates::find(doc) {
        Ok(file) => {
            let dest = scaffold::output_path(root, file.path());
            scaffold::write_file(&dest, file.contents(), force)?;
            Ok(())
        }
        Err(err) => Err(unknown_template_error(&err)),
    }
}

/// Create `docs/engineering/adrs/NNNN-<slug>.md` from the ADR template, where `NNNN` is the
/// next number after the highest existing record.
fn add_adr(root: &Path, slug: Option<&str>, force: bool) -> Result<()> {
    let slug = slug.context("`add adr` requires a slug, e.g. `docslime add adr my-decision`")?;
    let slug = normalize_slug(slug);
    if slug.is_empty() {
        bail!("the ADR slug must contain at least one alphanumeric character");
    }

    let docs_dir = root.join(scaffold::DOCS_DIR);
    let current_adr_dir = docs_dir.join(ADR_DIR);
    let legacy_adr_dir = docs_dir.join(LEGACY_ADR_DIR);
    let adr_dir = if current_adr_dir.exists() || !legacy_adr_dir.exists() {
        current_adr_dir
    } else {
        legacy_adr_dir
    };
    let next = next_adr_number(&adr_dir)?;
    let filename = format!("{next:04}-{slug}.md");
    let dest = adr_dir.join(&filename);

    scaffold::write_file(&dest, templates::ADR_TEMPLATE.as_bytes(), force)?;
    Ok(())
}

/// Scan an ADR directory for `NNNN-*` files and return the next sequence number.
///
/// Returns `1` if the directory doesn't exist yet or contains no numbered records.
fn next_adr_number(adr_dir: &Path) -> Result<u32> {
    let read = match std::fs::read_dir(adr_dir) {
        Ok(read) => read,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(1),
        Err(e) => {
            return Err(e).with_context(|| format!("failed to read {}", adr_dir.display()));
        }
    };

    let mut max = 0;
    for entry in read {
        let entry = entry?;
        if let Some(num) = leading_number(&entry.path()) {
            max = max.max(num);
        }
    }
    Ok(max + 1)
}

/// Extract the leading `NNNN` from a filename like `0007-foo.md`, if present.
fn leading_number(path: &Path) -> Option<u32> {
    let stem = path.file_name()?.to_str()?;
    let digits: String = stem.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

/// Lower-case, hyphenate, and strip a slug down to `[a-z0-9-]`.
fn normalize_slug(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut prev_dash = false;
    for ch in raw.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

/// Build a helpful error listing valid template names.
fn unknown_template_error(err: &FindError) -> anyhow::Error {
    let mut msg = format!("{err}\n\nAvailable templates:");
    for c in err.candidates() {
        msg.push_str(&format!("\n  {c}"));
    }
    msg.push_str("\n  adr <slug>   (creates the next-numbered ADR)");
    anyhow::anyhow!(msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_slug_lowercases_and_hyphenates() {
        assert_eq!(normalize_slug("My First Decision!"), "my-first-decision");
        assert_eq!(normalize_slug("  spaced  out  "), "spaced-out");
        assert_eq!(normalize_slug("already-good"), "already-good");
        assert_eq!(normalize_slug("CamelCase"), "camelcase");
        assert_eq!(normalize_slug("!!!"), "");
    }

    #[test]
    fn next_adr_number_is_one_when_dir_missing() {
        let tmp = std::env::temp_dir().join("docslime-no-such-dir-xyz");
        assert_eq!(next_adr_number(&tmp).unwrap(), 1);
    }

    #[test]
    fn leading_number_parses_prefix() {
        assert_eq!(leading_number(Path::new("0007-foo.md")), Some(7));
        assert_eq!(leading_number(Path::new("README.md")), None);
        assert_eq!(leading_number(Path::new("12-bar.md")), Some(12));
    }
}
