# Architecture

docgen is a single self-contained command-line binary written in Rust. It has no server, no
database, and no runtime dependencies: the entire template tree is compiled into the binary,
and every command operates directly on the filesystem relative to the current directory. The
design is deliberately small — parse a command, resolve a template, write files without
clobbering existing ones.

## Context diagram

```
developer ─┐
           ├─→ [ docgen CLI ] ─→ ./docs/ (files in the git repo)
AI agent  ─┘        │
                    └─ embedded templates (compiled in; no network, no external files)
```

Inside the boundary: argument parsing, template resolution, and file writing. Outside: the
git repo's filesystem (the only thing docgen touches) and the AI agent that later fills the
files in.

## Components

| Component | Responsibility | Depends on |
|---|---|---|
| `cli` | Define the command surface (`init`, `add`, `list`) and arguments via clap derive. | clap |
| `main` | Parse args, resolve the working directory, dispatch to a command, map errors to an exit code. | cli, commands |
| `commands::init` | Scaffold the full template tree into `docs/`, skipping existing files. | templates, scaffold |
| `commands::add` | Resolve a single template by name, or create the next-numbered ADR with a normalized slug. | templates, scaffold |
| `commands::list` | List every template and whether it already exists on disk. | templates, scaffold |
| `templates` | Hold the compile-time-embedded template tree and ADR template; resolve `add` names leniently. | include_dir |
| `scaffold` | Compute output paths and write files non-destructively (honoring `--force`); track outcomes. | std::fs |

## Data model

docgen is essentially stateless — it stores nothing of its own. The only persistent artifacts
are:

- **Embedded templates** — a read-only directory tree (`templates/`) plus the standalone ADR
  template (`assets/adr.md`), baked into the binary at build time.
- **The output `docs/` tree** — plain Markdown files written into the user's repo.

The one piece of derived state computed at runtime is the **next ADR number**, obtained by
scanning the ADR directory for the highest `NNNN-*` prefix.

## Key flows

### `docgen init` (scaffold the tree)

1. `main` resolves the current working directory as the root — `main`
2. Dispatch to the init command — `commands::init`
3. Enumerate every embedded template in the tree — `templates`
4. For each, compute `docs/<relative-path>` and write it unless it exists (or `--force`) — `scaffold`
5. Report a summary of created vs. skipped files — `commands::init`

### `docgen add adr <slug>` (create an ADR)

1. Dispatch to the add command; detect the literal `adr` — `commands::add`
2. Normalize the slug to `[a-z0-9-]`; reject if empty — `commands::add`
3. Scan `docs/2-ENGINEERING/ADRs/` for the highest `NNNN` prefix and add one — `commands::add`
4. Write `NNNN-<slug>.md` from the embedded ADR template, non-destructively — `scaffold`

These line up with the "Scaffold the docs tree" and "Record an architecture decision"
experiences in [`1-EXPERIENCES.md`](1-EXPERIENCES.md).

## Cross-cutting concerns

- **Error handling:** `anyhow` propagates errors up to `main`, which prints `error: …` and
  returns a non-zero `ExitCode`. Unknown template names produce a helpful error listing valid
  names (FR-8).
- **Configuration:** none — behavior is fixed by the embedded templates and a small set of
  flags (`--force`). No config file (see open questions in [`2-REQUIREMENTS.md`](2-REQUIREMENTS.md)).
- **Security:** docgen only writes within `docs/` under the current directory and never
  overwrites without `--force` (NFR-4). No network access.
- **Observability:** plain stdout/stderr; `list` uses `owo-colors` for readable status output.

## Decisions

- [Embed templates in the binary at compile time](2-ENGINEERING/ADRs/0001-embed-templates-in-binary.md) — keeps docgen a zero-dependency single binary.

## Risks & trade-offs

- **Editing a template requires a rebuild.** Because templates are embedded via `include_dir`,
  content changes only ship with a new binary. Accepted: it's the price of a no-dependency
  single binary, and templates change rarely.
- **Fixed tree layout.** docgen doesn't accept a custom structure, which keeps it simple but
  limits teams that want a different shape. Tracked as an open question in the requirements.
- **Current-directory assumption.** All commands act on the current directory; running from
  the wrong place scaffolds in the wrong place. Mitigation: the `docgen-init` skill confirms
  the working directory first.
