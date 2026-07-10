---
type: concept
title: Requirements
source: "https://www.docslime.dev/2-REQUIREMENTS/"
path: /2-REQUIREMENTS/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T04:25:52.802Z"
---
# Requirements

DocSlime must scaffold a fixed, standardized `docs/` tree into a git repo, add individual
documents (including auto-numbered ADRs) on demand, and report what exists without
destroying existing work. The templates must carry the inline guidance that lets an AI agent
fill them in, while the tree stays plain Markdown for `impeccable` context loading and
`docmd.io` publication.

## Functional requirements

| ID | Requirement | Traces to |
|---|---|---|
| FR-1 | The system shall create the full standardized `docs/` tree (`PRODUCT.md`, numbered docs, `DESIGN.md`, `publishing.md`, `0-PRODUCT/`, `1-JOURNEYS/`, `3-ENGINEERING/`, and `3-ENGINEERING/ADRs/`) in the current directory via `docslime init`. | Scaffold the docs tree |
| FR-2 | The system shall skip any file that already exists during `init` and `add`, and overwrite only when `--force` is given. | Non-destructive by default |
| FR-3 | The system shall add a single named document via `docslime add <name>`, resolving the name leniently (case-insensitive, with or without the `.md` extension). | Add a single document |
| FR-4 | The system shall create the next-numbered ADR via `docslime add adr <slug>`, where the number is one greater than the highest existing `NNNN-*` record in the ADR directory (`0001` if none). | Record an architecture decision |
| FR-5 | The system shall normalize an ADR slug to lowercase, hyphen-separated `[a-z0-9-]`, and reject a slug with no alphanumeric characters. | Record an architecture decision |
| FR-6 | The system shall list every available template and whether it already exists on disk via `docslime list`, including the `adr` entry. | Add a single document |
| FR-7 | The system shall embed all templates in the binary at compile time, requiring no network access or external files at runtime. | Fast and local |
| FR-8 | The system shall report a clear error and list valid template names when given an unknown document name, and exit non-zero. | Add a single document |
| FR-9 | The system shall provide installable agent skills (`docslime-install`, `docslime-init`, `docslime-fill`, `docslime-adr`, `docslime-kiss`) that drive the documentation lifecycle. | Fill in and improve documents with an agent |
| FR-10 | The `docslime-kiss` skill shall review filled docs for bloat, generic AI prose, stale contradictions, weak traceability, and non-actionable requirements, architecture, design, or testing content. | Tighten docs after filling |
| FR-11 | The generated tree shall include `docs/PRODUCT.md` and `docs/DESIGN.md` so `impeccable` can discover product and design context from the docs directory without duplicate root files. | Integrate with impeccable |
| FR-12 | KISS review shall remain an agent skill rather than a `docslime` CLI subcommand. | Tighten docs after filling |
| FR-13 | Documentation guidance shall make the quality trace explicit: product goal -> experience -> requirement -> BDD scenario -> test, with ADRs linked when a durable decision explains the design. | TDD+BDD quality |
| FR-14 | Architecture guidance shall ask for domain language and boundaries when useful, without forcing heavyweight Domain Driven Design ceremony on small projects. | Domain Driven Design |
| FR-15 | Publishing guidance shall point to official `docmd.io` build and deployment docs rather than copying that content into DocSlime. | Publishable Markdown |
| FR-16 | Each bundled DocSlime skill shall include clear prerequisites, steps, guardrails, verification, and failure handling. | Robust agent skills |

## Non-functional requirements

| ID | Requirement | Target / constraint |
|---|---|---|
| NFR-1 | Portability | Runs on macOS (arm64, x86_64) and Linux (arm64, x86_64); no runtime dependencies. |
| NFR-2 | Performance | Commands complete near-instantly (well under a second) on a typical repo. |
| NFR-3 | Distribution | Installable via Homebrew tap, shell installer script, and `cargo install`. |
| NFR-4 | Safety | No command destroys user data without an explicit `--force`. |
| NFR-5 | Maintainability | Templates are plain Markdown files in the repo, editable without touching Rust logic (rebuild required). |
| NFR-6 | Toolchain | Builds with Rust 1.74+ (2021 edition). |
| NFR-7 | Publication | Generated docs remain plain Markdown suitable for the `docmd.io` publishing system. |
| NFR-8 | Quality stance | Docs support TDD+BDD traceability, Domain Driven Design framing where useful, and ADR-backed decisions. |
| NFR-9 | Skill maintainability | Skill instructions stay compact enough for agents to follow while still naming setup, verification, and blocked-state behavior. |
| NFR-10 | Design context | `impeccable` context loading should be verified from `docs/PRODUCT.md` and `docs/DESIGN.md` whenever homepage or design docs change. |

## Constraints & assumptions

- **Constraint:** Editing a template requires recompiling the binary, since templates are embedded via `include_dir`.
- **Constraint:** The tree structure is fixed in the templates; DocSlime does not take a user-supplied layout.
- **Constraint:** `docslime-kiss` is a skill, not a CLI command.
- **Assumption:** `docslime` is run at the root of the target git repo; the tree is created relative to the current directory.
- **Assumption:** An AI coding agent (not the CLI itself) writes the document content by following the inline guidance.

## Dependencies

- **`clap`** — command-line argument parsing (derive API).
- **`include_dir`** — embeds the template tree into the binary at compile time.
- **`anyhow` / `thiserror`** — error handling and reporting.
- **`owo-colors`** — colored terminal output for `list`.
- **`cargo-dist`** — generates the release workflow and Homebrew formula (build/release time only).

## Open questions

- Should DocSlime support a config file or flags to customize the tree layout, or stay fixed? — maintainer
- Should `list` / `add` operate on a non-current directory via a `--path` flag? — maintainer
