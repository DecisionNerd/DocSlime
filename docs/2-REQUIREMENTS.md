# Requirements

docgen must scaffold a fixed, standardized `docs/` tree into a git repo, add individual
documents (including auto-numbered ADRs) on demand, and report what exists — all without
ever destroying existing work. The templates it writes must carry the inline guidance that
lets an AI agent fill them in. The tool ships as a single self-contained binary for macOS
and Linux.

## Functional requirements

| ID | Requirement | Traces to |
|---|---|---|
| FR-1 | The system shall create the full standardized `docs/` tree (numbered docs plus `0-PRODUCT/`, `1-JOURNEYS/`, `2-DESIGN/style-guide.md`, `3-ENGINEERING/`, and `3-ENGINEERING/ADRs/`) in the current directory via `docgen init`. | Scaffold the docs tree |
| FR-2 | The system shall skip any file that already exists during `init` and `add`, and overwrite only when `--force` is given. | Non-destructive by default |
| FR-3 | The system shall add a single named document via `docgen add <name>`, resolving the name leniently (case-insensitive, with or without the `.md` extension). | Add a single document |
| FR-4 | The system shall create the next-numbered ADR via `docgen add adr <slug>`, where the number is one greater than the highest existing `NNNN-*` record in the ADR directory (`0001` if none). | Record an architecture decision |
| FR-5 | The system shall normalize an ADR slug to lowercase, hyphen-separated `[a-z0-9-]`, and reject a slug with no alphanumeric characters. | Record an architecture decision |
| FR-6 | The system shall list every available template and whether it already exists on disk via `docgen list`, including the `adr` entry. | Add a single document |
| FR-7 | The system shall embed all templates in the binary at compile time, requiring no network access or external files at runtime. | Fast and local |
| FR-8 | The system shall report a clear error and list valid template names when given an unknown document name, and exit non-zero. | Add a single document |
| FR-9 | The system shall provide installable agent skills (`docgen-install`, `docgen-init`, `docgen-fill`, `docgen-adr`, `docgen-kiss`) that drive the documentation lifecycle. | Fill in and improve documents with an agent |
| FR-10 | The `docgen-kiss` skill shall review filled docs for bloat, generic AI prose, stale contradictions, weak traceability, and non-actionable requirements, architecture, design, or testing content. | Tighten docs after filling |

## Non-functional requirements

| ID | Requirement | Target / constraint |
|---|---|---|
| NFR-1 | Portability | Runs on macOS (arm64, x86_64) and Linux (arm64, x86_64); no runtime dependencies. |
| NFR-2 | Performance | Commands complete near-instantly (well under a second) on a typical repo. |
| NFR-3 | Distribution | Installable via Homebrew tap, shell installer script, and `cargo install`. |
| NFR-4 | Safety | No command destroys user data without an explicit `--force`. |
| NFR-5 | Maintainability | Templates are plain Markdown files in the repo, editable without touching Rust logic (rebuild required). |
| NFR-6 | Toolchain | Builds with Rust 1.74+ (2021 edition). |

## Constraints & assumptions

- **Constraint:** Editing a template requires recompiling the binary, since templates are embedded via `include_dir`.
- **Constraint:** The tree structure is fixed in the templates; docgen does not take a user-supplied layout.
- **Assumption:** docgen is run at the root of the target git repo; the tree is created relative to the current directory.
- **Assumption:** An AI coding agent (not docgen itself) writes the document content by following the inline guidance.

## Dependencies

- **`clap`** — command-line argument parsing (derive API).
- **`include_dir`** — embeds the template tree into the binary at compile time.
- **`anyhow` / `thiserror`** — error handling and reporting.
- **`owo-colors`** — colored terminal output for `list`.
- **`cargo-dist`** — generates the release workflow and Homebrew formula (build/release time only).

## Open questions

- Should docgen support a config file or flags to customize the tree layout, or stay fixed? — maintainer
- Should `list` / `add` operate on a non-current directory via a `--path` flag? — maintainer
