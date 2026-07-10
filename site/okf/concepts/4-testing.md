---
type: concept
title: Testing
source: "https://www.docslime.dev/4-TESTING/"
path: /4-TESTING/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T02:57:05.446Z"
---
# Testing

DocSlime is verified behavior-first: the things a user actually does — scaffold a tree, add a
document, create an ADR, list templates — are exercised end-to-end by running the real
compiled binary against throwaway directories and asserting on the files it produces. Small,
fiddly logic (slug normalization, ADR numbering) is covered by fast unit tests. The bar for
shippable is simple: the full suite is green, `impeccable` can discover docs context, and the
binary's observable behavior matches the experiences and requirements.

## Strategy

| Layer | What it verifies | Tools |
|---|---|---|
| Unit | Pure logic in isolation — slug normalization, next-ADR-number computation, filename prefix parsing. | Rust `#[test]` in `src/` |
| End-to-end / behavior | User-visible CLI behavior from [`1-EXPERIENCES.md`](1-EXPERIENCES.md) — running the actual binary and asserting on exit codes, output, and files written. | `assert_cmd`, `predicates`, `tempfile` |

There is no separate integration layer: the codebase is small and the components are best
verified together through the CLI, so the black-box tests double as integration coverage.

## Behavior coverage

| Experience / Requirement | Scenario (Given/When/Then) | Test |
|---|---|---|
| Scaffold the docs tree / FR-1, FR-11, FR-15 | Given an empty dir, When `init`, Then the full 12-file tree exists, including `PRODUCT.md`, `DESIGN.md`, and `publishing.md` | `tests/cli.rs::init_creates_full_tree` |
| Self-explaining templates | Given a scaffolded tree, When inspected, Then every template carries `<!-- LLM: -->` guidance | `tests/cli.rs::every_template_carries_llm_guidance` |
| Non-destructive by default / FR-2 | Given an edited file, When `init` re-runs, Then the file is left untouched | `tests/cli.rs::init_skips_existing_files` |
| Force overwrite / FR-2 | Given an edited file, When `init --force`, Then the template is restored | `tests/cli.rs::init_force_overwrites` |
| Add a single document / FR-3 | Given an empty dir, When `add <shorthand>`, Then the matching template is created | `tests/cli.rs::add_resolves_shorthand_name` |
| Unknown name errors / FR-8 | Given an unknown name, When `add`, Then it fails with a helpful error | `tests/cli.rs::add_unknown_template_fails` |
| Record an architecture decision / FR-4 | Given existing ADRs, When `add adr <slug>`, Then the next number is used | `tests/cli.rs::add_adr_numbers_sequentially` |
| ADR numbering from scratch / FR-4 | Given no ADR dir, When `add adr <slug>`, Then it starts at `0001` | `tests/cli.rs::add_adr_starts_at_one_without_init` |
| ADR slug required / FR-5 | Given no slug, When `add adr`, Then it fails | `tests/cli.rs::add_adr_requires_slug` |
| List templates / FR-6 | When `list`, Then every template is shown | `tests/cli.rs::list_shows_every_template` |
| List reflects disk state / FR-6 | Given some files exist, When `list`, Then status reflects what's on disk | `tests/cli.rs::list_reflects_on_disk_status` |
| Agent skill structure / FR-9, FR-10 | Given the bundled docslime skills, When validated, Then their frontmatter and metadata are well-formed | `skill-creator/scripts/quick_validate.py .agents/skills/<skill>` |
| KISS stays a skill / FR-12 | Given the CLI help, When inspected, Then there is no `kiss` CLI subcommand | `tests/cli.rs::kiss_is_not_a_cli_subcommand` |
| Skill robustness / FR-16 | Given the bundled skill folders, When inspected, Then each `SKILL.md` has frontmatter, guardrails, verification, and failure handling | `tests/cli.rs::agent_skills_have_required_sections` |
| Slug normalization / FR-5 | Mixed-case/spaced input normalizes to `[a-z0-9-]`; empty rejected | `src/commands/add.rs::normalize_slug_lowercases_and_hyphenates` |
| ADR numbering helper / FR-4 | Missing dir → `1`; filename prefix parsed correctly | `src/commands/add.rs::next_adr_number_is_one_when_dir_missing`, `leading_number_parses_prefix` |

## Traceability contract

DocSlime's TDD+BDD bar is that every important behavior can be followed from intent to
proof:

| Link | Evidence |
|---|---|
| Product goal -> experience | `PRODUCT.md` explains why the docs tree exists; `1-EXPERIENCES.md` turns that into user stories. |
| Experience -> requirement | `2-REQUIREMENTS.md` gives each behavior a stable FR/NFR ID and names the experience it serves. |
| Requirement -> BDD scenario | This file records Given/When/Then coverage for the visible behavior. |
| Scenario -> test | `tests/cli.rs` runs the compiled binary against throwaway directories and asserts on output files, exit codes, and help text. |
| Requirement -> architecture/ADR | `3-ARCHITECTURE.md` names the domain boundary; durable choices link to `3-ENGINEERING/ADRs/`. |

The DDD evidence is intentionally lightweight: docs should name the domain concepts and
boundaries that affect behavior, then use ADRs for hard-to-reverse decisions. If a project
has no meaningful domain split, the architecture doc should say that instead of inventing
bounded contexts.

## Evaluation against product goals

Tests prove correctness; they don't prove the product is working. The product-level signals from
[`PRODUCT.md`](PRODUCT.md) are evaluated qualitatively:

- **Docs get filled** — judged by whether scaffolded docs in real repos end up complete (no
  leftover `<!-- LLM: -->` guidance), rather than abandoned as templates. DocSlime dogfoods this
  by filling in its own `docs/` tree.
- **Low friction** — judged by the time and number of steps from `docslime init` to a first
  useful, filled-in document.
- **Agent context quality** — judged by whether agents working in a DocSlime-backed repo give less
  speculative answers because product context, requirements, and ADRs are present.
- **Impeccable context quality** — judged by whether `impeccable` resolves `docs/PRODUCT.md`
  and `docs/DESIGN.md` without duplicate root files.
- **Publishing boundary** — judged by whether DocSlime docs point to official `docmd.io`
  build/deploy guidance and avoid stale copied platform instructions.
- **Skill robustness** — judged by whether skill files validate structurally and give agents
  enough setup, guardrails, verification, and failure handling to act without guessing.
- **Adoption** — tracked via Homebrew and `npx skills` installs and repos using the tree.

## Running the tests

```sh
cargo test                  # unit + integration suite (expected: all green)
cargo clippy --all-targets  # lint (expected: no warnings)
npm run build               # docmd site build (expected: site/ output updates cleanly)
```

## Continuous integration

DocSlime is now a monorepo with three CI-owned surfaces:

| Workflow job | Surface | Gate |
|---|---|---|
| `Branch policy` | Release flow | Fails pull requests to `main` unless the source branch is `staging`; all feature work targets `staging`. |
| `CLI` | Rust binary and embedded templates | `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets -- -D warnings`. |
| `Site` | `docmd.io` documentation site | `npm ci` followed by `npm run build`. |
| `Agent skills` | Bundled `.agents/skills/docslime-*` package | Frontmatter, OpenAI metadata, guardrail, verification, and failure-handling checks. |

The daily CI workflow lives in `.github/workflows/ci.yml` and runs on pull requests to
`staging` or `main`, pushes to those branches, and manual dispatches. The release workflow
(`.github/workflows/release.yml`, generated by `cargo-dist`) remains tag-driven: it builds
and packages the CLI for all target platforms, creates the GitHub Release, and publishes the
Homebrew formula.

GitHub also has an active repository ruleset named "Protect main and staging from deletion".
It targets `refs/heads/main` and `refs/heads/staging` with the `deletion` rule so the two
release branches cannot be removed accidentally.

## Test data & environments

End-to-end tests run the real binary inside a fresh `tempfile::TempDir` per test, so each one
gets an isolated throwaway directory that is cleaned up automatically — no shared state, no
fixtures to seed, and no risk of touching the working repo.
