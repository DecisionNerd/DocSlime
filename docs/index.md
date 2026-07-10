---
title: "DocSlime"
description: "Opinionated docs scaffolding and agent skills for services and user-facing products."
---

::: hero layout:split glow:true
# Turn your repo into living docs.

DocSlime makes repo knowledge stick. It absorbs product intent, requirements, design,
architecture, tests, and decisions into one local docs body that humans and agents can use.

```sh
brew install DecisionNerd/tap/docslime
```

[Install details](#install-docslime){.docmd-button} [See the docs tree](#what-docslime-creates){.docmd-button .docmd-button-secondary} [View on GitHub](https://github.com/DecisionNerd/DocSlime){.docmd-button .docmd-button-ghost}

== side

```text
repo/
|-- docs/
|   |-- PRODUCT.md
|   |-- DESIGN.md
|   |-- 2-REQUIREMENTS.md
|   |-- 3-ARCHITECTURE.md
|   |-- 4-TESTING.md
|   |-- publishing.md
|   `-- 3-ENGINEERING/ADRs/
`-- .agents/skills/
    |-- docslime-fill
    |-- docslime-adr
    `-- docslime-kiss
```

:::

DocSlime is a small CLI plus a skill pack for creating, filling, reviewing, and publishing an
opinionated `docs/` tree. The name is silly on purpose; the method is not. It pulls the
scattered parts of a project into one integrated body, then gives future humans and AI agents
better context for the next change.

## Why It Sticks

```text
product intent  -> PRODUCT.md + DESIGN.md
behavior        -> REQUIREMENTS.md + TESTING.md
system shape    -> ARCHITECTURE.md + ADRs
maintenance     -> docslime-fill + docslime-adr + docslime-kiss
publication     -> docmd.io
```

- **Product and design context** flows into `PRODUCT.md` and `DESIGN.md`, so agents stop
  guessing what the code is for.
- **Requirements and behavior** flow into `2-REQUIREMENTS.md` and `4-TESTING.md`, so TDD+BDD
  work can trace back to intent.
- **Domain shape and tradeoffs** flow into `3-ARCHITECTURE.md` and `3-ENGINEERING/ADRs/`, so
  Domain Driven Design language stays close to decisions.
- **Human judgment** flows through `docslime-fill`, `docslime-adr`, and `docslime-kiss`, so
  docs get filled, decisions get recorded, and bloat gets cut.
- **Publishing stays thin**: DocSlime keeps the Markdown clean, then hands static-site build
  and hosting choices to the [`docmd.io` docs](publishing/).

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
|-- PRODUCT.md
|-- 1-EXPERIENCES.md
|-- 2-REQUIREMENTS.md
|-- DESIGN.md
|-- 3-ARCHITECTURE.md
|-- 4-TESTING.md
|-- publishing.md
|-- 0-PRODUCT/
|-- 1-JOURNEYS/
`-- 3-ENGINEERING/ADRs/
```

`docs/PRODUCT.md` and `docs/DESIGN.md` are deliberately discoverable from the docs tree so
tools like `impeccable` can load product and design context without duplicate root files.

- **Product + design:** `PRODUCT.md`, `DESIGN.md`, and `0-PRODUCT/` capture purpose,
  voice, principles, and success measures.
- **Experiences + requirements:** `1-EXPERIENCES.md`, `1-JOURNEYS/`, and
  `2-REQUIREMENTS.md` map human and agent journeys to testable behavior.
- **Architecture + ADRs:** `3-ARCHITECTURE.md` and `3-ENGINEERING/ADRs/` keep domain
  boundaries and decisions explicit.
- **Testing:** `4-TESTING.md` ties TDD and BDD coverage back to requirements and journeys.
- **Publishing:** [`publishing.md`](publishing/) points to the official `docmd.io` build and
  deployment docs instead of copying that system into DocSlime.

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
[Understand the product model](PRODUCT/){.docmd-button .docmd-button-secondary}
:::
:::

::: grid
::: card "Experiences" icon:route
[See the core workflows](1-EXPERIENCES/){.docmd-button .docmd-button-secondary}
:::
:::

::: grid
::: card "Requirements" icon:list-checks
[Review the behavior contract](2-REQUIREMENTS/){.docmd-button .docmd-button-secondary}
:::
:::

::: grid
::: card "Design" icon:palette
[Open the design context](DESIGN/){.docmd-button .docmd-button-secondary}
:::
:::

::: grid
::: card "Publishing" icon:upload-cloud
[Ship through docmd.io](publishing/){.docmd-button .docmd-button-secondary}
:::
:::
:::
