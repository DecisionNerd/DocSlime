---
name: docslime-init
description: Scaffolds and tailors the DocSlime docs starting template with docslime init. Use when setting up project docs, initializing docs in a repo, or safely adding missing default docs.
---

# DocSlime Init

Scaffold a broad `docs/` starting tree into the current git repo so product context, continuous discovery, requirements, design guidance, architecture, testing, delivery, observability, and decisions can live alongside the code. Tailor the result to the project instead of treating every generated file as mandatory.

## When to Use

- "Set up docs for this project"
- "Scaffold the docs tree"
- "Initialize DocSlime here"
- Starting documentation on a repo that has no `docs/` directory yet

## Prerequisites

The `docslime` binary must be installed. If `docslime --version` fails, run the **docslime-install** skill first.

## Guardrails

- Confirm you are at the target git repo root before writing files.
- `docslime init` is non-destructive by default; do not use `--force` unless the user asks to overwrite existing scaffold files.
- Treat the scaffold as a template, not a compliance checklist. Recommend irrelevant files for removal or consolidation and get confirmation before deleting them.
- When `docs/PRODUCT.md` and `docs/DESIGN.md` are useful, keep them in `docs/`; do not create duplicate root bridge files for tools such as `impeccable`.
- Remember the CLI surface is `init`, `add`, and `list`. KISS review belongs to the **docslime-kiss** skill.

## Steps

### 1 — Confirm the working directory

Make sure you're at the root of the target git repo. The tree is created relative to the current directory.

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

Existing files are **never overwritten**. `init` also recognizes the old numbered paths and treats them as legacy matches rather than creating duplicate renamed docs. To intentionally write the new template path, re-run with `docslime init --force`; this still does not remove the legacy file.

### 3 — Tailor the template

Identify the project type and the people or agents who consume it, then keep only documents that reduce ambiguity for those users. Link to an authoritative organization-level source instead of duplicating it locally.

For example, a backend API service in a large organization may not need local product strategy or visual design docs when those concerns are owned elsewhere. It may still keep `experience/` for developer experience, operator and integration journeys, and agent experience when coding agents or other automated consumers use the API.

When removing or merging template files, update `docs/README.md` and affected links so the remaining tree is intentional and navigable. Preserve real evidence and accepted decisions.

### 4 — See what's there

```bash
docslime list
```

`list` shows every template and whether it already exists on disk, plus the `adr` entry.

### 5 — Orient and hand off

The docs form a lifecycle: strategy -> product/design -> discovery -> requirements -> architecture -> testing -> publishing -> observability -> discovery. Each template contains inline `<!-- LLM: ... -->` guidance for interviewing the user and filling the sections.

Tell the user the tailored tree is ready, and offer to start filling the first applicable document with the **docslime-fill** skill. Begin with `docs/PRODUCT.md` only when project-level product context belongs in this repo; otherwise start with the most relevant experience, requirements, or architecture document.

## Verification

```bash
docslime list
test -f docs/PRODUCT.md && test -f docs/DESIGN.md
```

Confirm created/skipped/legacy output is clear and that the retained tree matches the project's real consumers. Missing template files are acceptable when their omission is intentional and `docs/README.md` reflects the final shape.

## Failure Handling

- If `docslime` is missing, stop and run **docslime-install** first.
- If the current directory is not a git repo or appears to be the wrong repo, ask before scaffolding.
- If files already exist, report that `init` skipped them; do not overwrite unless the user explicitly approves `docslime init --force`.
