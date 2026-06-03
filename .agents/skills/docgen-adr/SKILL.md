---
name: docgen-adr
description: Create the next-numbered Architecture Decision Record with docgen and fill it in by interviewing the user about one decision, then update the decision log.
---

# docgen ADR

Record a single architecture decision as an ADR — the context, the options, the choice, and
its consequences — so the reasoning lives in the repo alongside the code.

## When to Use

- "Record this decision"
- "Write an ADR for choosing Postgres"
- "Document why we picked X over Y"
- "Add an architecture decision record"
- Any time a significant, hard-to-reverse technical choice is being made

## Prerequisites

The `docs/` tree must exist (run **docgen-init** if not) and `docgen` must be installed (run
**docgen-install** if not). ADRs live in `docs/2-ENGINEERING/ADRs/`.

## Steps

### 1 — Create the next-numbered record

```bash
docgen add adr <short-slug>
```

`<slug>` is a short kebab name for the decision, e.g. `use-postgres`. docgen writes
`docs/2-ENGINEERING/ADRs/NNNN-<slug>.md`, where `NNNN` is the next number after the highest
existing record (`0001` if there are none). The slug is lower-cased and hyphenated
automatically, so `docgen add adr "Use Postgres"` produces `0002-use-postgres.md`.

If you don't know the slug yet, ask the user for a 2–4 word name for the decision first.

### 2 — Fill it in by interviewing about ONE decision

Open the new file and follow its inline `<!-- LLM: ... -->` guidance. Keep it focused — one
ADR, one decision. Work through the sections:

- **Title & metadata** — set `ADR-NNNN: <Title>`, `Status: Proposed` (→ `Accepted` once
  decided), today's `Date`, and the deciders.
- **Context** — the forces, constraints, and requirements that force a choice. Reference
  requirement IDs from `../../2-REQUIREMENTS.md` where relevant. State facts, not the choice.
- **Options considered** — the realistic alternatives (including "do nothing"), each with
  pros and cons. Ask the user what was actually on the table.
- **Decision** — the option chosen and the reasoning. Be definite.
- **Consequences** — positive, negative, and follow-up. Be honest about the trade-offs.

Ask one focused question at a time and reflect answers back. Remove each `<!-- LLM: ... -->`
comment and replace each `_italic prompt_` as you complete its section.

### 3 — Update the decision log

Add a row for this ADR to the table in `docs/2-ENGINEERING/ADRs/README.md`:

```
| 0002 | Use Postgres | Accepted | 2026-06-02 |
```

Keep the log in sync — one row per ADR file.

### 4 — Treat accepted ADRs as immutable

Once an ADR is `Accepted`, don't rewrite it. To change the decision, create a **new** ADR
that supersedes it, and set the old one's status to `Superseded by ADR-NNNN`.

### 5 — Verify and report

```bash
grep -rn "LLM:" docs/2-ENGINEERING/ADRs/
```

When the new ADR and the log are clean, summarize the decision and the record's number.
