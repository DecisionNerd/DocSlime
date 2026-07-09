---
type: concept
title: DocSlime
description: "Opinionated docs scaffolding and agent skills for services and user-facing products."
source: "https://www.docslime.dev/"
path: /
updated: 2026-07-09
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-09T20:10:32.450Z"
---
---
title: "DocSlime"
description: "Opinionated docs scaffolding and agent skills for services and user-facing products."
---

# DocSlime

DocSlime turns a repo into a living, agent-ready documentation workspace. It gives teams a
small `docslime` CLI, a standardized `docs/` tree, and agent skills that help fill, review,
and maintain product context, requirements, design, architecture, testing notes, and ADRs.

The system is built for both services and user-facing products. It is intentionally
opinionated: TDD+BDD quality, Domain Driven Design where it clarifies the system,
explicit decisions, and plain Markdown that can publish through `docmd.io`.

## Install

```sh
brew install DecisionNerd/tap/docslime
```

Other install paths:

- Shell installer: `curl -LsSf https://github.com/DecisionNerd/DocSlime/releases/latest/download/docslime-installer.sh | sh`
- Source build: `cargo install --git https://github.com/DecisionNerd/DocSlime --bins`

## Use the CLI

```sh
docslime init
docslime list
docslime add PRODUCT
docslime add adr choose-storage-boundary
```

`init` creates the full tree without overwriting existing files. `add` creates one missing
document or the next-numbered ADR. `list` shows every available template and whether it
already exists in the current repo.

## What DocSlime Creates

```text
docs/
├── PRODUCT.md
├── 1-EXPERIENCES.md
├── 2-REQUIREMENTS.md
├── DESIGN.md
├── 3-ARCHITECTURE.md
├── 4-TESTING.md
├── 0-PRODUCT/
├── 1-JOURNEYS/
└── 3-ENGINEERING/ADRs/
```

`docs/PRODUCT.md` and `docs/DESIGN.md` are deliberately discoverable from the docs tree so
tools like `impeccable` can load product and design context without duplicate root files.

## Agent Skills

Install the bundled skills into a skill-aware agent:

```sh
npx skills add DecisionNerd/DocSlime
```

The skill pack includes `docslime-install`, `docslime-init`, `docslime-fill`,
`docslime-adr`, and `docslime-kiss`.

## Read Next

- [Product](PRODUCT/) explains what DocSlime is for.
- [Experiences](1-EXPERIENCES/) describes the user and agent workflows.
- [Requirements](2-REQUIREMENTS/) defines the behavior DocSlime must preserve.
- [Design](DESIGN/) captures the product and docs experience rules.
- [Agent Skills](skills/) shows how agents use the DocSlime lifecycle.
