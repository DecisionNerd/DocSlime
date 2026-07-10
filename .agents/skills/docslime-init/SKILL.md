---
name: docslime-init
description: Scaffolds the standardized DocSlime docs tree with docslime init. Use when setting up project docs, initializing docs in a repo, or safely adding missing default docs.
---

# DocSlime Init

Scaffold the full `docs/` tree into the current git repo so product context, continuous
discovery, requirements, design guidance, architecture, testing, delivery, observability,
and decisions live alongside the code.

## When to Use

- "Set up docs for this project"
- "Scaffold the docs tree"
- "Initialize DocSlime here"
- Starting documentation on a repo that has no `docs/` directory yet

## Prerequisites

The `docslime` binary must be installed. If `docslime --version` fails, run the
**docslime-install** skill first.

## Guardrails

- Confirm you are at the target git repo root before writing files.
- `docslime init` is non-destructive by default; do not use `--force` unless the user asks to
  overwrite existing scaffold files.
- Keep `docs/PRODUCT.md` and `docs/DESIGN.md` in `docs/`; do not create duplicate root bridge
  files for tools such as `impeccable`.
- Remember the CLI surface is `init`, `add`, and `list`. KISS review belongs to the
  **docslime-kiss** skill.

## Steps

### 1 — Confirm the working directory

Make sure you're at the root of the target git repo. The tree is created relative to the
current directory.

```bash
pwd && ls
```

### 2 — Scaffold the tree

```bash
docslime init
```

This creates:

```
docs/
├── README.md           # how the docs are organized
├── PRODUCT.md          # product context and goals
├── DESIGN.md           # product design / style / accessibility guidance
├── REQUIREMENTS.md     # build contract derived from discovery
├── strategy/           # market, positioning, roadmap, strategic bets
├── experience/         # continuous discovery and experience design
└── engineering/
    ├── README.md       # engineering lifecycle index
    ├── ARCHITECTURE.md # domain and system design
    ├── TESTING.md      # testing strategy and CI evidence
    ├── PUBLISHING.md   # continuous delivery and rollback
    ├── OBSERVABILITY.md # production health and product learning
    └── adrs/           # Architecture Decision Records
```

Existing files are **never overwritten**. `init` also recognizes the old numbered paths and
treats them as legacy matches rather than creating duplicate renamed docs. To intentionally
write the new template path, re-run with `docslime init --force`; this still does not remove
the legacy file.

### 3 — See what's there

```bash
docslime list
```

`list` shows every template and whether it already exists on disk, plus the `adr` entry.

### 4 — Orient and hand off

The docs form a lifecycle: strategy -> product/design -> discovery -> requirements ->
architecture -> testing -> publishing -> observability -> discovery. Each template contains
inline `<!-- LLM: ... -->` guidance for interviewing the user and filling the sections.

Tell the user the tree is ready, and offer to start filling it in with the **docslime-fill**
skill — beginning with `docs/PRODUCT.md`, since every other document derives from it.

## Verification

```bash
docslime list
test -f docs/PRODUCT.md && test -f docs/DESIGN.md
```

Confirm created/skipped/legacy output is clear and that the tree includes requirements,
continuous discovery, the engineering lifecycle, observability, and ADRs.

## Failure Handling

- If `docslime` is missing, stop and run **docslime-install** first.
- If the current directory is not a git repo or appears to be the wrong repo, ask before
  scaffolding.
- If files already exist, report that `init` skipped them; do not overwrite unless the user
  explicitly approves `docslime init --force`.
