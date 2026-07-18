# Requirements

DocSlime must scaffold a fixed, standardized `docs/` tree into a git repo, add individual documents (including auto-numbered ADRs) on demand, and report what exists without destroying existing work. The templates must carry the inline guidance that lets an AI agent fill them in, while the tree stays plain Markdown for design-context loading, continuous discovery, delivery, publication, and production learning.

## Functional requirements

| ID | Requirement | Traces to |
| --- | --- | --- |
| FR-1 | The system shall create the full standardized `docs/` tree with `README.md`, `PRODUCT.md`, `DESIGN.md`, `REQUIREMENTS.md`, `strategy/`, `experience/`, and engineering architecture, testing, publishing, observability, and ADR docs via `docslime init`. | Scaffold the docs tree |
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
| FR-13 | Documentation guidance shall make the lifecycle trace explicit: strategy -> product/design -> discovery -> requirement -> architecture -> test -> publish -> observe -> discovery, with ADRs linked when a durable decision explains the design. | Trace intent through production |
| FR-14 | Architecture guidance shall ask teams to model the real-world problem through its meaningful concepts, relationships, constraints, rules, workflows, lifecycle states, preferred terminology, and responsibility boundaries without requiring ceremonial abstractions. | Domain modeling |
| FR-15 | Publishing guidance shall cover artifacts, versioning, CI gates, promotion, deployment verification, rollback, and official platform references without copying volatile provider instructions. | Publish verified artifacts |
| FR-16 | Each bundled DocSlime skill shall include clear prerequisites, steps, guardrails, verification, and failure handling. | Robust agent skills |
| FR-17 | When a legacy numbered path exists, `init` and `list` shall recognize it as satisfying the corresponding current template and shall not create a duplicate renamed file unless `--force` is explicit. | Preserve existing documentation during upgrades |
| FR-18 | Experience guidance shall define a continuous-discovery practice and connect evidence, journeys, opportunities, hypotheses, behavior scenarios, and success signals to requirements. | Learn from users continuously |
| FR-19 | Observability guidance shall connect production system health and user-outcome signals to requirements and discovery while covering telemetry, SLOs, alerts, ownership, and privacy. | Close the product-learning loop |
| FR-20 | Documentation guidance and first-party docs shall render flowcharts as Mermaid diagrams rather than ASCII art. | Keep diagrams readable and renderable |
| FR-21 | The scaffold and bundled skills shall describe the docs tree as a project-specific starting template, permit intentional omission of irrelevant product, strategy, or design docs, and preserve experience documentation for developer and agent consumers when useful. | Fit the docs to the project |
| FR-22 | Publishing guidance shall present Semantic Versioning and Conventional Commits as optional practices, preserve effective existing workflows, and prohibit adding enforcement or rewriting history without explicit team agreement. | Suggest release conventions without imposing workflow |

## Non-functional requirements

| ID | Requirement | Target / constraint |
| --- | --- | --- |
| NFR-1 | Portability | Runs on macOS (arm64, x86_64) and Linux (arm64, x86_64); no runtime dependencies. |
| NFR-2 | Performance | Commands complete near-instantly (well under a second) on a typical repo. |
| NFR-3 | Distribution | Installable via Homebrew tap, shell installer script, and `cargo install`. |
| NFR-4 | Safety | No command destroys user data without an explicit `--force`. |
| NFR-5 | Maintainability | Templates are plain Markdown files in the repo, editable without touching Rust logic (rebuild required). |
| NFR-6 | Toolchain | Builds with Rust 1.74+ (2021 edition). |
| NFR-7 | Publication | Generated docs remain plain Markdown suitable for the `docmd.io` publishing system. |
| NFR-8 | Quality stance | Docs connect the real-world problem and shared model to product decisions, implementation, TDD+BDD verification, and ADR-backed decisions. |
| NFR-9 | Skill maintainability | Skill instructions stay compact enough for agents to follow while still naming setup, verification, and blocked-state behavior. |
| NFR-10 | Design context | `impeccable` context loading should be verified from `docs/PRODUCT.md` and `docs/DESIGN.md` whenever homepage or design docs change. |
| NFR-11 | Privacy | The default observability guidance must not normalize collecting sensitive repository or user data without an explicit decision, consent, and controls. |

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
