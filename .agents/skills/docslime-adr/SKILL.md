---
name: docslime-adr
description: Creates and fills the next-numbered DocSlime Architecture Decision Record. Use when recording a significant product or technical decision, documenting why an option was chosen, or adding an ADR.
---

# DocSlime ADR

Record a single architecture decision as an ADR — the context, the options, the choice, and its consequences — so the reasoning lives in the repo alongside the code.

## When to Use

- "Record this decision"
- "Write an ADR for choosing Postgres"
- "Document why we picked X over Y"
- "Add an architecture decision record"
- Any time a significant, hard-to-reverse technical choice is being made

## Prerequisites

The `docs/` tree must exist (run **docslime-init** if not) and `docslime` must be installed (run **docslime-install** if not). ADRs live in `docs/engineering/adrs/`. Older initialized trees may still use `docs/3-ENGINEERING/ADRs/`; the CLI continues there rather than splitting the log.

## Guardrails

- Record one decision per ADR. Do not use an ADR to relitigate the whole system.
- Keep accepted ADRs immutable; create a new superseding ADR for changed decisions.
- Tie context back to requirement IDs, domain boundaries, constraints, or forces when they are known.
- Ask the user what options were actually considered; do not invent alternatives for polish.

## Steps

### 1 — Create the next-numbered record

```bash
docslime add adr <short-slug>
```

`<slug>` is a short kebab name for the decision, e.g. `use-postgres`. `docslime` writes `docs/engineering/adrs/NNNN-<slug>.md`, where `NNNN` is the next number after the highest existing record (`0001` if there are none). The slug is lower-cased and hyphenated automatically, so `docslime add adr "Use Postgres"` produces `0002-use-postgres.md`.

If you don't know the slug yet, ask the user for a 2–4 word name for the decision first.

### 2 — Fill it in by interviewing about ONE decision

Open the new file and follow its inline `<!-- LLM: ... -->` guidance. Keep it focused — one ADR, one decision. Work through the sections:

- **Title & metadata** — set `ADR-NNNN: <Title>`, `Status: Proposed` (→ `Accepted` once decided), today's `Date`, and the deciders.
- **Context** — the forces, constraints, and requirements that force a choice. Reference requirement IDs from `../../REQUIREMENTS.md` where relevant. State facts, not the choice.
- **Options considered** — the realistic alternatives (including "do nothing"), each with pros and cons. Ask the user what was actually on the table.
- **Decision** — the option chosen and the reasoning. Be definite.
- **Consequences** — positive, negative, and follow-up. Be honest about the trade-offs.

Ask one focused question at a time and reflect answers back. Remove each `<!-- LLM: ... -->` comment and replace each `_italic prompt_` as you complete its section.

### 3 — Update the decision log

Add a row for this ADR to the table in `docs/engineering/adrs/README.md` (or the legacy ADR index when the CLI deliberately continued an older tree):

```
| 0002 | Use Postgres | Accepted | 2026-06-02 |
```

Keep the log in sync — one row per ADR file.

### 4 — Treat accepted ADRs as immutable

Once an ADR is `Accepted`, don't rewrite it. To change the decision, create a **new** ADR that supersedes it, and set the old one's status to `Superseded by ADR-NNNN`.

### 5 — Verify and report

See the verification section below before reporting completion.

## Verification

```bash
grep -rn "LLM:" docs/engineering/adrs/ docs/3-ENGINEERING/ADRs/ 2>/dev/null
```

When the new ADR and the log are clean, summarize the decision and the record's number.

Also confirm the ADR file exists at the expected `NNNN-<slug>.md` path and the ADR index has one row for that file.

## Failure Handling

- If the docs tree or ADR directory is missing, run **docslime-init** or create the missing ADR path through `docslime add adr <slug>`.
- If the slug is unknown, ask for a 2-4 word decision name before creating the record.
- If the decision is not actually made, keep the ADR status `Proposed` and list the unresolved question rather than forcing `Accepted`.
