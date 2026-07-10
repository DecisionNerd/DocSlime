---
type: concept
title: Design
source: "https://www.docslime.dev/DESIGN/"
path: /DESIGN/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T04:22:37.538Z"
---
# Design

DocSlime has no graphical app today, but it still has a product experience: the CLI, the
generated Markdown, the agent skills, and the way those docs later publish through
`docmd.io`. This file is the design context that tools like `impeccable` can load from
`docs/`.

## Design Principles

- **Useful before decorative:** the first run should leave the user with a clear, fillable
  docs tree and obvious next action.
- **Opinionated, not heavy:** DocSlime should make strong defaults feel helpful, not like a
  governance framework.
- **Agent-readable and human-readable:** templates should guide an agent without making a
  human feel like they are reading internal machinery.
- **Traceable by design:** requirements, BDD scenarios, tests, architecture, and ADRs should
  naturally point at each other.
- **Small command surface:** CLI commands stay memorable; judgment-heavy work belongs in
  skills.

## Brand And Voice

- **Product name:** use `DocSlime` for the product and `docslime` for the binary/command.
- **Tone:** practical, direct, and lightly playful when the name earns it. The product can
  carry the integrated "slime" idea without turning docs into a joke.
- **Mascot:** use the blue anime slime as a product signal. It can appear in the homepage
  hero and brand surfaces, but it should support the repo/docs artifact rather than replacing
  developer credibility.
- **Terminology:** use "docs tree," "product context," "design context," "requirements,"
  "BDD," "Domain Driven Design," "ADR," "template," "skill," and "agent" consistently.
- **Writing rules:** generated guidance should be specific and interview-driven. Prefer
  "ask the user," "trace this to a requirement," and "record the decision" over vague advice.

## Impeccable Workflow

`impeccable` should treat `docs/PRODUCT.md` and `docs/DESIGN.md` as the current product and
design context. UI work should begin by loading those files, critiquing the actual surface,
and then changing the smallest set of docs/site files that makes the critique more true.

For the homepage, the desired direction is:

- lead with the outcome: DocSlime turns a repo into living docs;
- show the repo/docs tree as the credibility anchor;
- include the blue slime mascot as a memorable brand moment;
- keep CTAs developer-native and direct;
- avoid pink/purple gradient text, decorative grid backgrounds, oversized cards, and claims
  that `docslime-kiss` is a CLI command.

## Markdown Experience

- `docs/PRODUCT.md` and `docs/DESIGN.md` are top-level context files so design and coding
  agents can discover them from the docs directory.
- Numbered docs keep the reading path obvious after product/design context:
  experiences, requirements, architecture, and testing.
- Tables are useful for traceability, requirements, component lists, and ADR logs; prose
  should stay short enough to scan.
- Templates may contain `<!-- LLM: ... -->` guidance, but filled docs should remove it.
- Avoid generic motivational copy. Every paragraph should help a future human or agent make
  a better change.

## CLI Experience

- **Commands:** keep `docslime init`, `docslime add`, and `docslime list` as the CLI surface.
- **Skills:** keep `docslime-kiss` as an agent skill, not a CLI command.
- **Defaults:** commands skip existing files unless `--force` is explicit.
- **States:** every write should report a clear created, skipped, or overwritten outcome.
- **Errors:** failures should say what happened and, where possible, list valid alternatives.
- **Color:** terminal color is functional only; status words must carry meaning without it.

## Quality Bar

- Requirements should be testable and carry stable IDs.
- Testing docs should map requirements and experiences to Given/When/Then behavior coverage.
- Architecture docs should describe domain concepts and boundaries when the project has them.
- Significant product or technical decisions should be captured as ADRs.
- KISS review should cut bloat, stale contradictions, generic AI prose, and weak traceability.
- Docs should remain plain Markdown that can publish through `docmd.io` without cleanup.

## References

- [`src/scaffold.rs`](../src/scaffold.rs) - write status output and safety behavior.
- [`src/commands/list.rs`](../src/commands/list.rs) - template list output.
- [`templates/`](../templates/) - embedded Markdown style for generated docs.
- [`.agents/skills/`](../.agents/skills/) - lifecycle skills for filling, ADRs, and KISS review.
