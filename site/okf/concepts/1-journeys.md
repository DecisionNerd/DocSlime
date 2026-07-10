---
type: concept
title: Journeys
source: "https://www.docslime.dev/1-JOURNEYS/"
path: /1-JOURNEYS/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T02:05:24.380Z"
---
# Journeys

This folder expands on [`../1-EXPERIENCES.md`](../1-EXPERIENCES.md) with full user personas
and end-to-end journey maps.

## Personas

### Team developer

- **Who:** an engineer on a team that wants consistent, in-repo project docs, typically working alongside an AI coding agent.
- **Goals:** stand up documentation quickly, in a structure that's the same across their repos, without designing a doc system from scratch.
- **Frustrations:** project intent is scattered across wikis, chat, and tickets; every project documents things differently; docs go stale or never get written.
- **Success looks like:** one command produces a familiar tree, and their agent fills it in with real content that lives next to the code.

### AI coding agent

- **Who:** the assistant working inside the repo (e.g. via the DocSlime skills), acting as both author and consumer of the docs.
- **Goals:** understand the project's intent so it can act with full context; produce documentation that reflects what the team actually wants.
- **Frustrations:** without recorded intent it has to guess from code; ad-hoc doc structures give it nothing reliable to read or write against.
- **Success looks like:** clear inline guidance tells it what to ask and how to write each section, so it interviews the team and leaves a clean, complete document.

## Journeys

### Document a project from scratch

Traces to the "Scaffold the docs tree" and "Fill in a document with an agent" experiences in
[`../1-EXPERIENCES.md`](../1-EXPERIENCES.md).

| Step | User does | User feels | Opportunity |
|---|---|---|---|
| 1 | Runs `docslime init` in the repo | Curious, low commitment | Produce the full tree instantly, non-destructively |
| 2 | Sees the numbered `docs/` tree appear | Oriented — the structure is obvious | Numbering signals the reading order |
| 3 | Asks the agent to fill in `PRODUCT.md` | Slightly unsure what to write | Inline guidance turns it into a guided product interview |
| 4 | Answers the agent's focused questions | Engaged; thinking is being captured | Reflect answers back; write one tight section at a time |
| 5 | Reviews the completed doc, commits it | Satisfied — real intent, in the repo | Clean output with no leftover guidance comments |
| 6 | Moves down the chain to the next doc | Momentum | Each doc builds on the last, so context compounds |

## Index

_No standalone persona or journey files yet. Add one file per substantial persona or major
journey as the need arises, and list it here._
