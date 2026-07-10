---
type: concept
title: "Architecture Decision Records"
source: "https://www.docslime.dev/3-ENGINEERING/ADRs/"
path: /3-ENGINEERING/ADRs/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T03:11:12.357Z"
---
# Architecture Decision Records

An **Architecture Decision Record (ADR)** captures one significant decision — the context,
the choice made, and its consequences — so the reasoning lives in the repo alongside the
code. Decisions are immutable once accepted: to change one, add a new ADR that supersedes it.

## Creating an ADR

```
docslime add adr <short-slug>
```

This creates the next-numbered record, e.g. `0001-<short-slug>.md`. Fill it in (the file
carries inline guidance), then add a row to the log below.

## Status values

- **Proposed** — under discussion.
- **Accepted** — decided and in effect.
- **Superseded by ADR-NNNN** — replaced by a later decision.
- **Deprecated** — no longer relevant.

## Decision log

| ADR | Title | Status | Date |
|---|---|---|---|
| [0001](0001-embed-templates-in-binary.md) | Embed templates in the binary at compile time | Accepted | 2026-06-02 |
