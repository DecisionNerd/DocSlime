---
type: concept
title: "ADR-0001: Embed templates in the binary at compile time"
source: "https://www.docslime.dev/3-ENGINEERING/ADRs/0001-embed-templates-in-binary/"
path: /3-ENGINEERING/ADRs/0001-embed-templates-in-binary/
updated: 2026-07-09
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-09T20:10:32.449Z"
---
# ADR-0001: Embed templates in the binary at compile time

- **Status:** Accepted
- **Date:** 2026-06-02
- **Deciders:** DecisionNerd (maintainer)

## Context

DocSlime's job is to write a tree of Markdown templates into a user's repo. Those templates
have to reach the user's machine somehow. The tool's requirements call for a portable,
zero-runtime-dependency binary that works offline (NFR-1, FR-7) and installs cleanly via
Homebrew, a shell script, or `cargo install` (NFR-3). Whatever mechanism delivers the
templates must not require the user to also have a separate templates directory present, or
network access, at runtime.

## Options considered

### Option A — Embed templates in the binary at compile time
- **Pros:** Single self-contained artifact; works offline; nothing to locate or install
  alongside the binary; trivial to distribute via Homebrew / shell installer / `cargo install`.
- **Cons:** Editing a template requires recompiling and re-releasing; templates can't be
  customized by the user without a rebuild.

### Option B — Ship templates as separate data files installed next to the binary
- **Pros:** Templates editable in place; could allow user overrides without recompiling.
- **Cons:** Must reliably locate the data directory across platforms and install methods;
  packaging is more complex; risk of the binary and its templates getting out of sync or the
  data files going missing.

### Option C — Fetch templates from a remote source at runtime
- **Pros:** Templates updatable without releasing a new binary.
- **Cons:** Requires network access (violates the offline requirement); adds latency and a
  failure mode; introduces a hosting dependency for a tool that should be instant and local.

## Decision

We will embed the entire template tree in the binary at compile time, using `include_dir!`
for the `init` tree and `include_str!` for the standalone ADR template. This makes DocSlime a
single self-contained binary with no runtime dependencies, satisfying the portability,
offline, and distribution requirements directly.

## Consequences

- **Positive:** DocSlime ships as one artifact that runs anywhere with no setup, no network,
  and nothing to locate at runtime — which keeps installation and usage friction low.
- **Negative:** Any template change requires recompiling and cutting a new release; users
  cannot customize the templates without rebuilding from source.
- **Follow-up:** If user-customizable layouts become a requirement (tracked as an open
  question in `../../2-REQUIREMENTS.md`), revisit with a new ADR that supersedes this one —
  e.g. allowing an external template directory to override the embedded default.
