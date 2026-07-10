---
title: "DocSlime"
description: "Opinionated docs scaffolding and agent skills for services and user-facing products."
image: /assets/images/docslime-hero.png
---

::: hero glow:true
# Turn your repo into living docs.

DocSlime makes repo knowledge stick. It connects product intent and continuous discovery to
requirements, architecture, tests, delivery, observability, and decisions in one local docs
body that humans and agents can use.

```sh
brew install DecisionNerd/tap/docslime
```

[Install DocSlime](#install-docslime){.docmd-button} [See the docs tree](#what-docslime-creates){.docmd-button} [View on GitHub](https://github.com/DecisionNerd/DocSlime){.docmd-button}

:::

DocSlime is a small CLI plus a skill pack for creating, filling, reviewing, and publishing an
opinionated `docs/` tree. The name is silly on purpose; the method is not. It pulls the
scattered parts of a project into one integrated body, then gives future humans and AI agents
better context for the next change.

## Why It Sticks

```text
strategy -> product/design -> discovery -> requirements -> architecture -> testing
    ^                                                               |
    +----------- observation <- publishing <- verified build <------+
```

- **Product and design context** flows into `PRODUCT.md` and `DESIGN.md`, so agents stop
  guessing what the code is for.
- **Continuous discovery** lives in `experience/`, where evidence and journeys become
  solution-neutral requirements rather than an untraceable feature backlog.
- **Requirements and behavior** flow into `REQUIREMENTS.md` and `engineering/TESTING.md`, so
  TDD+BDD work can trace back to evidence.
- **Domain shape and tradeoffs** flow into `engineering/ARCHITECTURE.md` and `engineering/adrs/`, so
  Domain Driven Design language stays close to decisions.
- **Human judgment** flows through `docslime-fill`, `docslime-adr`, and `docslime-kiss`, so
  docs get filled, decisions get recorded, and bloat gets cut.
- **Delivery and observation close the loop**: `engineering/PUBLISHING.md` defines promotion,
  verification, and rollback; `engineering/OBSERVABILITY.md` connects production health and
  user outcomes back to discovery.

## Install DocSlime

Homebrew is the recommended local install path:

```sh
brew install DecisionNerd/tap/docslime
```

::: callout tip "Safe by default"
`docslime init` creates missing docs and reports what was created or skipped. It will not
overwrite existing files unless `--force` is explicit. The slime eats context, not your
worktree.
:::

Other install paths stay available when Homebrew is not the right fit.

```sh
curl -LsSf \
  https://github.com/DecisionNerd/DocSlime/releases/latest/download/docslime-installer.sh \
  | sh
```

```sh
cargo install \
  --git https://github.com/DecisionNerd/DocSlime \
  --bins
```

## First Run Path

::: grids
::: grid
::: card "1. Initialize" icon:terminal
```sh
docslime init
```

Create the standard docs tree and leave clear next steps in the repo.
:::
:::

::: grid
::: card "2. Fill" icon:messages-square
```sh
npx skills add DecisionNerd/DocSlime
```

Use `docslime-fill` to interview the team and replace scaffold guidance with real context.
:::
:::

::: grid
::: card "3. Decide" icon:scroll-text
```sh
docslime add adr choose-storage-boundary
```

Record significant product and technical choices while they are still fresh.
:::
:::

::: grid
::: card "4. Keep It Lean" icon:scissors
Run `docslime-kiss` as an agent skill to find bloat, contradictions, stale placeholders, and
weak traceability before docs become ceremony.
:::
:::
:::

The recommended happy path is short: install the CLI, run `docslime init`, add the skill
pack, fill `docs/PRODUCT.md`, then use `docslime-kiss` once the first useful context exists.

## What DocSlime Creates

```text
docs/
|-- README.md
|-- PRODUCT.md
|-- DESIGN.md
|-- REQUIREMENTS.md
|-- strategy/README.md
|-- experience/README.md
`-- engineering/
    |-- README.md
    |-- ARCHITECTURE.md
    |-- TESTING.md
    |-- PUBLISHING.md
    |-- OBSERVABILITY.md
    `-- adrs/README.md
```

`docs/PRODUCT.md` and `docs/DESIGN.md` are deliberately discoverable from the docs tree so
tools like `impeccable` can load product and design context without duplicate root files.

- **Product + design:** `PRODUCT.md`, `DESIGN.md`, and `strategy/` capture purpose,
  voice, principles, and success measures.
- **Experience + requirements:** `experience/` captures evidence, opportunities, journeys,
  and hypotheses; `REQUIREMENTS.md` translates them into a testable build contract.
- **Architecture + ADRs:** `engineering/ARCHITECTURE.md` and `engineering/adrs/` keep domain
  boundaries and decisions explicit.
- **Testing:** `engineering/TESTING.md` ties TDD and BDD coverage back to requirements and journeys.
- **Publishing + observability:** [`engineering/PUBLISHING.md`](engineering/PUBLISHING/) and
  [`engineering/OBSERVABILITY.md`](engineering/OBSERVABILITY/) carry verified artifacts to
  users and feed production evidence back into discovery.

## Agent Skills

DocSlime keeps judgment-heavy work in skills instead of pretending every review belongs in a
CLI subcommand.

- `docslime-install` and `docslime-init` verify the CLI and scaffold the docs tree without
  overwriting existing work.
- `docslime-fill` interviews the user, replaces scaffold guidance, and keeps facts anchored
  in the repo.
- `docslime-adr` creates the next-numbered ADR and writes the decision in the project
  vocabulary.
- `docslime-kiss` reviews for contradictions, generic AI prose, weak traceability, and
  overgrown docs.

## Read Next

::: grids
::: grid
::: card "Product" icon:target
[Understand the product model](PRODUCT/){.docmd-button}
:::
:::

::: grid
::: card "Experience" icon:route
[See continuous discovery](experience/){.docmd-button}
:::
:::

::: grid
::: card "Requirements" icon:list-checks
[Review the behavior contract](REQUIREMENTS/){.docmd-button}
:::
:::

::: grid
::: card "Design" icon:palette
[Open the design context](DESIGN/){.docmd-button}
:::
:::

::: grid
::: card "Engineering" icon:wrench
[Follow the engineering lifecycle](engineering/){.docmd-button}
:::
:::
:::
