---
type: concept
title: Product
source: "https://www.docslime.dev/PRODUCT/"
path: /PRODUCT/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T04:25:52.803Z"
---
# Product

DocSlime is an opinionated documentation system for codebases. It gives a repo a consistent
`docs/` tree, a `docslime` CLI, and agent skills that help teams turn product intent into
filled-in, version-controlled Markdown.

It exists so services and user-facing products can carry their product context, experiences,
requirements, design guidance, architecture, tests, and decisions next to the code. Agents
working in the repo can read that context directly instead of guessing from implementation
details alone.

## Problem

Project knowledge usually lives away from the work: in tickets, chats, wikis, docs tools,
and memory. That is painful for humans and worse for AI coding agents, which often see the
code but not the intent behind it. Teams also reinvent documentation structure per repo, so
there is no familiar shape for agents to read, fill, tighten, or publish.

The result is predictable: missing context, stale decisions, requirements that do not map to
tests, and architecture docs that sound polished but do not guide the next change.

## Vision

DocSlime makes the repo the product and engineering context container. A team runs
`docslime init`, answers focused agent questions, records ADRs as decisions happen, and uses
`docslime-kiss` to keep the tree lean and honest.

The system is playful in name but serious in method: it gathers the scattered pieces of a
project into one integrated docs body, then gives humans and agents more power because the
context is complete, local, and testable.

## Product Shape

- **CLI:** `docslime init`, `docslime add`, and `docslime list` manage the scaffold.
- **Skills:** `docslime-install`, `docslime-init`, `docslime-fill`, `docslime-adr`, and
  `docslime-kiss` drive the human-in-the-loop documentation lifecycle.
- **Impeccable integration:** `docs/PRODUCT.md` and `docs/DESIGN.md` are discoverable product
  and design context files without needing root-level duplicates, so `impeccable` can critique
  and polish product surfaces from the same docs tree agents use.
- **Publishing path:** the docs stay plain Markdown and are intended to publish through the
  `docmd.io` system; DocSlime links to that system instead of reimplementing it.
- **Quality stance:** requirements should trace to BDD scenarios and tests, architecture
  should use Domain Driven Design language where it clarifies the system, and significant
  choices should become ADRs.

## Quality Method

DocSlime's quality method is a trace, not a ceremony:

```text
product goal -> experience -> requirement -> BDD scenario -> test -> ADR when a choice matters
```

- **TDD+BDD:** requirements get stable IDs, behavior is written in Given/When/Then language,
  and tests prove observable behavior before the docs call it done.
- **Domain Driven Design:** architecture docs name the domain concepts, boundaries, and
  language that matter for the project; small projects can stay light, but they should still
  know their core nouns and responsibilities.
- **ADRs:** durable choices live in `3-ENGINEERING/ADRs/` so future humans and agents can see
  why the current shape exists.

## Goals

- Give every repo a consistent, agent-readable documentation structure.
- Make first-run value fast: `docslime init` should create useful next steps immediately.
- Keep product, design, engineering, testing, and ADR context local and version-controlled.
- Support both services and user-facing products without splitting into separate doc systems.
- Make docs easier to fill, review, tighten, and publish than to let drift.

## Non-goals

- DocSlime does not replace the human conversation. Agents fill docs by interviewing the
  team, not by inventing facts.
- DocSlime does not add a runtime service or database to the target project.
- DocSlime does not turn the CLI into a publishing host; publication belongs to the
  `docmd.io` system.
- DocSlime does not make `kiss` a CLI command. KISS review is an agent skill because it
  depends on judgment over filled docs.

## Success Metrics

- **Docs get filled:** scaffolded documents are completed, with no leftover `LLM:` guidance.
- **Traceability improves:** requirements, behavior scenarios, tests, and ADRs point to each
  other clearly enough for a future agent to act.
- **Design context improves:** `impeccable` resolves `docs/PRODUCT.md` and `docs/DESIGN.md`
  as the current product/design source without bridge files that can drift.
- **Low friction:** short time from `docslime init` to a useful first filled document.
- **Adoption:** installs through Homebrew, shell installer, source builds, and skill-pack use.
- **Publication readiness:** docs remain clean Markdown that can flow into `docmd.io`.

## Stakeholders

- **Maintainer (DecisionNerd):** owns DocSlime's direction, release process, templates, and
  skill pack.
- **Engineering teams:** adopt the tree across repos to keep intent close to code.
- **Product and design collaborators:** use `PRODUCT.md` and `DESIGN.md` as compact context
  for shaping user-facing work.
- **AI coding agents:** fill, consume, critique, and maintain the docs while working in the
  repo.
