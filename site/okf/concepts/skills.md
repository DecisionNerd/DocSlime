---
type: concept
title: "Agent Skills"
description: "Teach AI coding agents to install, initialize, fill, review, and maintain DocSlime docs."
source: "https://www.docslime.dev/skills/"
path: /skills/
updated: 2026-07-09
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-09T20:10:32.450Z"
---
---
title: "Agent Skills"
description: "Teach AI coding agents to install, initialize, fill, review, and maintain DocSlime docs."
---

# Agent Skills

DocSlime ships agent skills so Codex, Claude Code, ChatGPT, and other skill-aware tools can
drive the documentation lifecycle with the same product rules the CLI scaffolds.

The CLI creates files. The skills do the judgment-heavy work: check installation, scaffold a
repo, interview the user, fill documents, create ADRs, and review the result for KISS
problems.

## Install

```sh
npx skills add DecisionNerd/DocSlime
```

For a specific coding agent:

```sh
npx skills add DecisionNerd/DocSlime --agent codex
npx skills add DecisionNerd/DocSlime --agent claude-code
```

## Skills

| Skill | What it does |
|---|---|
| `docslime-install` | Verifies the `docslime` binary and installs it if missing. |
| `docslime-init` | Confirms the current repo and scaffolds the standard `docs/` tree. |
| `docslime-fill` | Interviews the user and fills one scaffolded document. |
| `docslime-adr` | Creates and fills the next-numbered Architecture Decision Record. |
| `docslime-kiss` | Reviews filled docs for bloat, stale contradictions, weak traceability, and generic AI prose. |

## How it works at runtime

Once installed, invoke the skills from inside the agent:

```text
/docslime-install
/docslime-init
/docslime-fill PRODUCT.md
/docslime-adr embed-templates-in-binary
/docslime-kiss
```

Each skill follows the docs tree instead of maintaining a separate source of truth. Product
context lives in `docs/PRODUCT.md`, design context lives in `docs/DESIGN.md`, and decisions
live in `docs/3-ENGINEERING/ADRs/`.

## Recommended flow

1. Run `docslime init`.
2. Fill `PRODUCT.md` with `docslime-fill`.
3. Work through experiences, requirements, design, architecture, and testing.
4. Use `docslime-adr` whenever a durable technical or product decision is made.
5. Run `docslime-kiss` before merging documentation-heavy changes.
