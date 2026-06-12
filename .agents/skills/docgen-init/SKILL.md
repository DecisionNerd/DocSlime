---
name: docgen-init
description: Scaffolds the standardized docgen docs tree with docgen init. Use when setting up project docs, initializing docs in a repo, or safely adding missing default docs.
---

# docgen Init

Scaffold the full `docs/` tree into the current git repo so the project's mission,
experiences, requirements, design guidance, architecture, and decisions live alongside the
code.

## When to Use

- "Set up docs for this project"
- "Scaffold the docs tree"
- "Initialize docgen here"
- Starting documentation on a repo that has no `docs/` directory yet

## Prerequisites

The `docgen` binary must be installed. If `docgen --version` fails, run the
**docgen-install** skill first.

## Steps

### 1 — Confirm the working directory

Make sure you're at the root of the target git repo. The tree is created relative to the
current directory.

```bash
pwd && ls
```

### 2 — Scaffold the tree

```bash
docgen init
```

This creates:

```
docs/
├── README.md           # how the docs are organized
├── 0-MISSION.md        # why the project exists
├── 1-EXPERIENCES.md    # the user experience
├── 2-REQUIREMENTS.md   # what the system must do
├── 3-ARCHITECTURE.md   # how it is designed
├── 4-TESTING.md        # how we prove it works
├── 0-PRODUCT/          # product / market / mission detail
├── 1-JOURNEYS/         # personas and user journeys
├── 2-DESIGN/           # product design / style / accessibility guidance
│   └── style-guide.md  # default style guide
└── 3-ENGINEERING/      # technical docs
    └── ADRs/           # Architecture Decision Records
```

Existing files are **never overwritten** — `init` skips anything already on disk. To
intentionally overwrite, re-run with `docgen init --force`.

### 3 — See what's there

```bash
docgen list
```

`list` shows every template and whether it already exists on disk, plus the `adr` entry.

### 4 — Orient and hand off

The numbered documents read as a chain, each building on the one before it. Each file
contains inline `<!-- LLM: ... -->` comments telling you what to interview the user about
and how to write each section.

Tell the user the tree is ready, and offer to start filling it in with the **docgen-fill**
skill — beginning with `docs/0-MISSION.md`, since every other document derives from it.
