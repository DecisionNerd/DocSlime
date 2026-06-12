# Style Guide

docgen's style guide is mostly about product shape, writing, and terminal behavior. It does
not have a graphical product UI today, but it still needs a consistent experience for CLI
commands, generated docs, and agent-facing instructions.

## Design Principles

- **Quiet and direct** — the tool should feel like it gets out of the way: one command,
  predictable files, clear output.
- **Agent-readable by default** — generated docs should guide an AI coding agent without
  making humans feel like they are reading machine instructions.
- **Non-destructive confidence** — every interaction should make it obvious that existing
  user work is safe unless `--force` is explicit.
- **Consistent over clever** — the tree should become familiar across repos; avoid per-project
  novelty in the default scaffold.

## Brand & Voice

- **Tone:** calm, practical, and plainspoken. docgen should sound like a helpful engineering
  tool, not a marketing site or a heavyweight governance framework.
- **Terminology:** use "docs tree," "template," "ADR," "guidance," and "scaffold"
  consistently. Use "agent" for the AI assistant expected to fill the templates.
- **Writing rules:** keep generated guidance imperative and specific. Prefer "ask the user"
  and "link to the source" over abstract advice.

## Visual Style

docgen has no web UI, design tokens, or component library. Its visible style is terminal
output and Markdown structure.

- **Color:** terminal color is functional only: green for created/existing success, dim for
  skipped or missing, yellow for overwrite, cyan for the ADR helper entry.
- **Typography:** generated Markdown uses clear sentence case headings and compact tables
  where comparison helps.
- **Spacing & layout:** template files should stay scannable, with short sections and one
  primary idea per paragraph.
- **Iconography & imagery:** not applicable for the CLI or default docs.

## Interaction Patterns

- **Commands:** keep the command surface small and memorable: `init`, `add`, and `list`.
- **Defaults:** commands skip existing files unless `--force` is present.
- **States:** every file write reports one visible state: create, skip, or overwrite.
- **Errors:** failures should tell the user what went wrong and, where possible, list valid
  alternatives.
- **Motion:** not applicable.

## Components

| Component / pattern | Use it for | Notes / source |
|---|---|---|
| Status line | Per-file write feedback | Implemented in `src/scaffold.rs` |
| Template index | Showing available templates and disk state | Implemented in `src/commands/list.rs` |
| Inline `LLM` guidance | Teaching agents how to fill a doc | Present in every template file |
| ADR log | Tracking decision records | `docs/3-ENGINEERING/ADRs/README.md` |

## Accessibility

- CLI output must remain understandable without color; status words carry the meaning.
- Markdown docs should use semantic headings in order and avoid layout that only works when
  visually scanned.
- Generated guidance should be explicit enough for screen-reader users and AI agents to
  understand without relying on visual proximity alone.

## References

- [`src/scaffold.rs`](../../src/scaffold.rs) — write status output and safety behavior.
- [`src/commands/list.rs`](../../src/commands/list.rs) — template list output.
- [`templates/`](../../templates/) — embedded Markdown style for generated docs.
