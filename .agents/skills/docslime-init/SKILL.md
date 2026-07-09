---
name: docslime-init
description: Scaffolds the standardized DocSlime docs tree with docslime init. Use when setting up project docs, initializing docs in a repo, or safely adding missing default docs.
---

# DocSlime Init

Scaffold the full `docs/` tree into the current git repo so the project's product context,
experiences, requirements, design guidance, architecture, tests, and decisions live
alongside the code.

## When to Use

- "Set up docs for this project"
- "Scaffold the docs tree"
- "Initialize DocSlime here"
- Starting documentation on a repo that has no `docs/` directory yet

## Prerequisites

The `docslime` binary must be installed. If `docslime --version` fails, run the
**docslime-install** skill first.

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
├── 1-EXPERIENCES.md    # the user experience
├── 2-REQUIREMENTS.md   # what the system must do
├── DESIGN.md           # product design / style / accessibility guidance
├── 3-ARCHITECTURE.md   # how it is designed
├── 4-TESTING.md        # how we prove it works
├── 0-PRODUCT/          # product / market / positioning detail
├── 1-JOURNEYS/         # personas and user journeys
└── 3-ENGINEERING/      # technical docs
    └── ADRs/           # Architecture Decision Records
```

Existing files are **never overwritten** — `init` skips anything already on disk. To
intentionally overwrite, re-run with `docslime init --force`.

### 3 — See what's there

```bash
docslime list
```

`list` shows every template and whether it already exists on disk, plus the `adr` entry.

### 4 — Orient and hand off

The top-level documents read as a chain, each building on the one before it. Each file
contains inline `<!-- LLM: ... -->` comments telling you what to interview the user about
and how to write each section.

Tell the user the tree is ready, and offer to start filling it in with the **docslime-fill**
skill — beginning with `docs/PRODUCT.md`, since every other document derives from it.
