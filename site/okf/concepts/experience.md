---
type: concept
title: Experience
source: "https://www.docslime.dev/experience/"
path: /experience/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T20:50:08.952Z"
---
# Experience

This folder is DocSlime's continuous-discovery and experience-design workspace. It connects
observed user needs to requirements without turning research notes into an untraceable
feature backlog.

## Discovery practice

DocSlime is maintainer-led today. Product learning comes from direct maintainer and user
feedback, issues and pull requests, real repositories using the scaffold and skills, CLI
behavior observed during development, and whether agents can act accurately from the
generated docs. Claims that lack evidence remain hypotheses rather than being written as
validated user facts.

A finding becomes a requirement when it identifies an observable behavior or measurable
quality the product must provide. A requirement remains solution-neutral; implementation
choices belong in architecture or an ADR.

## Primary participants

- **Team developer:** wants a familiar in-repo documentation system without inventing one
  for every project.
- **Product or design collaborator:** needs concise product and design context that tools can
  discover without maintaining duplicate files.
- **AI coding agent:** needs evidence, requirements, boundaries, and decisions close to the
  code so it can ask focused questions instead of fabricating context.

## Core journeys

| Journey | Desired outcome | Requirements |
|---|---|---|
| Scaffold a repo | One safe command creates the complete lifecycle-oriented docs tree. | FR-1, FR-2, FR-7 |
| Add one missing document | A partial tree can gain one known template without overwriting work. | FR-2, FR-3, FR-8 |
| Record a decision | The next collision-free ADR is created with a normalized memorable slug. | FR-4, FR-5 |
| Fill docs with an agent | The agent interviews the team, follows inline guidance, and removes scaffolding. | FR-9, FR-16 |
| Tighten docs | Review cuts bloat, contradictions, weak requirements, and invented certainty. | FR-10, FR-12 |
| Load design context | `impeccable` resolves canonical `docs/PRODUCT.md` and `docs/DESIGN.md`. | FR-11 |
| Trace intent through production | Discovery, requirements, architecture, tests, delivery, and observability point to one another. | FR-13–FR-18 |

## Experience principles

- **Non-destructive by default:** existing user-authored files are never overwritten or
  migrated implicitly.
- **Evidence before assertion:** agents ask rather than inventing users, systems, metrics, or
  decisions.
- **Opinionated, not ceremonial:** the lifecycle is complete, but documents stay as small as
  the project permits.
- **Traceable quality:** a user need can be followed to a requirement, test, decision,
  release, and production signal.
- **Fast and local:** the CLI works offline against the current repository.
- **Discoverable context:** standard names and a semantic tree make important context easy
  for humans, agents, and publishing tools to find.

## Continuous-discovery artifact shape

Create one focused lowercase-kebab-case file when an opportunity, study, journey, or product
slice needs more evidence than this index can hold. Use only the sections the evidence earns:

```markdown
# Opportunity or experience

## Observed need and evidence
## Desired user and business outcome
## Users and context
## Current journey
## Opportunity and hypothesis
## Intended behavior
## Given / When / Then scenarios
## Constraints and domain language
## Success signals and telemetry
## Open questions
## Related requirements, tests, architecture, and ADRs
```

## Learning loop

```text
evidence -> opportunity -> requirement -> architecture -> test -> release -> observation
     ^                                                                    |
     +--------------------------------------------------------------------+
```

## Index

No separate discovery artifacts exist yet. Add them when evidence for a meaningful
opportunity or journey should outlive the issue or conversation where it surfaced.
