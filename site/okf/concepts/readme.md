---
type: concept
title: Documentation
source: "https://www.docslime.dev/README/"
path: /README/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T04:22:37.538Z"
---
# Documentation

This folder holds the living documentation for DocSlime. It is structured for
**behavior-driven development**: product and design context lead into experiences,
requirements, architecture, testing, and ADRs. The docs stay in-repo so both people and AI
coding agents have full context without reaching for an external source.

## How the docs are organized

Read the top-level docs in this order:

| Document | Question it answers |
|---|---|
| [`PRODUCT.md`](PRODUCT.md) | What is this product, who is it for, and why does it exist? |
| [`1-EXPERIENCES.md`](1-EXPERIENCES.md) | What should it feel like to use? |
| [`2-REQUIREMENTS.md`](2-REQUIREMENTS.md) | What must the system do? |
| [`DESIGN.md`](DESIGN.md) | What should stay consistent in product, docs, and CLI experience? |
| [`3-ARCHITECTURE.md`](3-ARCHITECTURE.md) | How is the system built? |
| [`4-TESTING.md`](4-TESTING.md) | How do we prove it works? |
| [`publishing.md`](publishing.md) | How does the Markdown tree hand off to `docmd.io`? |

Supporting detail lives in subfolders:

| Folder | Contents |
|---|---|
| [`0-PRODUCT/`](0-PRODUCT/) | Product, market, and positioning detail beyond `PRODUCT.md`. |
| [`1-JOURNEYS/`](1-JOURNEYS/) | User personas, journeys, and experience detail. |
| [`3-ENGINEERING/`](3-ENGINEERING/) | Technical documentation, including testing and decision records. |
| [`3-ENGINEERING/ADRs/`](3-ENGINEERING/ADRs/) | Architecture Decision Records. |

## Conventions

- **Keep docs current.** When behavior changes, update the doc in the same change.
- **Link, don't duplicate.** Reference detail in subfolders rather than copying it.
- **Decisions are recorded.** Significant choices get an ADR (see `3-ENGINEERING/ADRs/`).
- **Keep context discoverable.** `PRODUCT.md` and `DESIGN.md` stay in `docs/` so tools like
  `impeccable` can load them without duplicate root files.
- **Keep publishing thin.** DocSlime prepares Markdown; `docmd.io` owns build and deployment
  behavior.
