//! Template inventory: the markdown templates embedded into the binary at compile time.
//!
//! The `templates/` directory at the crate root mirrors the output `docs/` tree exactly.
//! `include_dir!` walks it at build time so the binary has no runtime dependency on those
//! files. The single-record ADR template lives outside that tree (it isn't part of `init`)
//! and is embedded separately.

use include_dir::{include_dir, Dir, File};
use std::path::Path;

/// The full `docs/` template tree, embedded at compile time.
static TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// The single-record ADR template, used by `docslime add adr <slug>`.
pub const ADR_TEMPLATE: &str = include_str!("../assets/adr.md");

/// Every template file in the tree, in a stable (sorted-by-path) order.
///
/// `Dir::files()` is not recursive, so we walk `entries()` depth-first ourselves.
pub fn all() -> Vec<&'static File<'static>> {
    let mut out = Vec::new();
    collect(&TEMPLATES, &mut out);
    out.sort_by_key(|f| f.path());
    out
}

fn collect(dir: &'static Dir<'static>, out: &mut Vec<&'static File<'static>>) {
    for file in dir.files() {
        out.push(file);
    }
    for sub in dir.dirs() {
        collect(sub, out);
    }
}

/// Relative paths (within the template tree) of every template, sorted.
pub fn relative_paths() -> Vec<&'static str> {
    all().iter().filter_map(|f| f.path().to_str()).collect()
}

/// Legacy scaffold paths that correspond to a current template path.
///
/// These paths are not emitted by new releases. They are recognized so running a newer
/// `init` against a filled pre-rename tree does not create duplicate documents beside the
/// user's existing work.
pub fn legacy_paths(relative: &Path) -> &'static [&'static str] {
    match relative.to_str() {
        Some("strategy/README.md") => &["0-PRODUCT/README.md"],
        Some("experience/README.md") => &["1-EXPERIENCES.md", "1-JOURNEYS/README.md"],
        Some("REQUIREMENTS.md") => &["2-REQUIREMENTS.md"],
        Some("engineering/ARCHITECTURE.md") => &["3-ARCHITECTURE.md"],
        Some("engineering/TESTING.md") => &["4-TESTING.md"],
        Some("engineering/PUBLISHING.md") => &["publishing.md"],
        Some("engineering/README.md") => &["3-ENGINEERING/README.md"],
        Some("engineering/adrs/README.md") => &["3-ENGINEERING/ADRs/README.md"],
        _ => &[],
    }
}

/// Resolve a user-supplied `add <doc>` argument to a single template file.
///
/// Resolution is tried in order:
/// 1. exact relative-path match (`engineering/adrs/README.md`)
/// 2. relative path with the `.md` extension optional (`PRODUCT` -> `PRODUCT.md`)
/// 3. unambiguous case-insensitive basename match (`PRODUCT` -> `PRODUCT.md`)
///
/// Returns `Err` with the list of candidate paths when the name is unknown or ambiguous.
pub fn find(name: &str) -> Result<&'static File<'static>, FindError> {
    let files = all();

    // Preserve the old command vocabulary after the scaffold path rename. Directory names
    // also resolve to their README so `docslime add experience` remains intuitive.
    let normalized = name.trim_end_matches(".md").to_ascii_lowercase();
    let alias = match normalized.as_str() {
        "0-product" | "0-product/readme" | "strategy" | "strategy/readme" => {
            Some("strategy/README.md")
        }
        "1-experiences" | "1-journeys" | "1-journeys/readme" | "experience"
        | "experience/readme" => Some("experience/README.md"),
        "2-requirements" => Some("REQUIREMENTS.md"),
        "3-architecture" => Some("engineering/ARCHITECTURE.md"),
        "4-testing" => Some("engineering/TESTING.md"),
        "publishing" => Some("engineering/PUBLISHING.md"),
        "3-engineering" | "3-engineering/readme" | "engineering" | "engineering/readme" => {
            Some("engineering/README.md")
        }
        "3-engineering/adrs/readme" | "engineering/adrs" | "engineering/adrs/readme" => {
            Some("engineering/adrs/README.md")
        }
        _ => None,
    };
    if let Some(path) = alias {
        if let Some(file) = files.iter().find(|file| file.path().to_str() == Some(path)) {
            return Ok(file);
        }
    }

    // 1. exact relative path
    if let Some(f) = files.iter().find(|f| f.path().to_str() == Some(name)) {
        return Ok(f);
    }

    // 2. relative path with optional `.md`
    let with_md = format!("{name}.md");
    if let Some(f) = files
        .iter()
        .find(|f| f.path().to_str() == Some(with_md.as_str()))
    {
        return Ok(f);
    }

    // 3. case-insensitive basename match (with or without `.md`)
    let needle = normalized;
    let basename_matches: Vec<&'static File<'static>> = files
        .iter()
        .copied()
        .filter(|f| {
            f.path()
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|stem| stem.eq_ignore_ascii_case(&needle))
                .unwrap_or(false)
        })
        .collect();

    match basename_matches.as_slice() {
        [one] => Ok(one),
        [] => Err(FindError::NotFound {
            name: name.to_string(),
            candidates: relative_paths().iter().map(|s| s.to_string()).collect(),
        }),
        many => Err(FindError::Ambiguous {
            name: name.to_string(),
            candidates: many
                .iter()
                .filter_map(|f| f.path().to_str().map(str::to_string))
                .collect(),
        }),
    }
}

/// Error resolving a template name in [`find`].
#[derive(Debug, thiserror::Error)]
pub enum FindError {
    #[error("no template matches '{name}'")]
    NotFound {
        name: String,
        candidates: Vec<String>,
    },
    #[error("'{name}' is ambiguous")]
    Ambiguous {
        name: String,
        candidates: Vec<String>,
    },
}

impl FindError {
    /// The candidate paths to suggest to the user.
    pub fn candidates(&self) -> &[String] {
        match self {
            FindError::NotFound { candidates, .. } => candidates,
            FindError::Ambiguous { candidates, .. } => candidates,
        }
    }
}
