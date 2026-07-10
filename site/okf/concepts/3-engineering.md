---
type: concept
title: Engineering
source: "https://www.docslime.dev/3-ENGINEERING/"
path: /3-ENGINEERING/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T03:11:12.357Z"
---
# Engineering

This folder holds the deeper technical documentation behind
[`../3-ARCHITECTURE.md`](../3-ARCHITECTURE.md) and [`../4-TESTING.md`](../4-TESTING.md),
including the project's decision records.

## What lives here

DocSlime is a small Rust CLI plus agent skill pack, so the core engineering detail is
captured in the architecture and testing docs plus the project [`README`](../../README.md).
The topics below are the natural homes for additional detail as it's needed:

- **Development setup** — `cargo build` / `cargo test` / `cargo clippy`; see the Development section of the project README.
- **Build & release** — `cargo-dist` builds the binaries and Homebrew formula on a tagged release; see the Releasing section of the README and `.github/workflows/release.yml`.
- **API / interface reference** — the CLI surface (`init`, `add`, `list`) is documented via `docslime --help`.
- **[ADRs/](ADRs/)** — Architecture Decision Records (one per significant decision).

## Decision records

Significant engineering and product decisions are recorded as ADRs in [`ADRs/`](ADRs/).
Create the next one with:

```
docslime add adr <short-slug>
```

## Index

_No standalone engineering documents yet beyond the decision records in [`ADRs/`](ADRs/).
Add focused files per topic above as the need arises, and list them here._
