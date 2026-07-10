---
type: concept
title: Documentation
source: "https://www.docslime.dev/README/"
path: /README/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T20:50:08.948Z"
---
# Documentation

DocSlime dogfoods the documentation lifecycle it scaffolds. Product and design context feed
continuous discovery, requirements translate that evidence into a build contract, and the
engineering docs carry the contract through architecture, testing, publishing, and
production observability.

## Lifecycle

| Document | Question it answers |
|---|---|
| [`PRODUCT.md`](PRODUCT.md) | What is DocSlime, who is it for, and why does it exist? |
| [`DESIGN.md`](DESIGN.md) | What should stay consistent across its CLI, templates, skills, and site? |
| [`experience/`](experience/) | What user evidence, journeys, and behaviors inform the product? |
| [`REQUIREMENTS.md`](REQUIREMENTS.md) | What must DocSlime demonstrably do as a result? |
| [`engineering/ARCHITECTURE.md`](engineering/ARCHITECTURE.md) | How is the system built? |
| [`engineering/TESTING.md`](engineering/TESTING.md) | How do tests and CI prove it before release? |
| [`engineering/PUBLISHING.md`](engineering/PUBLISHING.md) | How do verified CLI, skill, and site artifacts reach users? |
| [`engineering/OBSERVABILITY.md`](engineering/OBSERVABILITY.md) | How do we verify releases and learn from production signals? |

Supporting detail lives in:

| Folder | Contents |
|---|---|
| [`strategy/`](strategy/) | Market, positioning, roadmap, and strategic bets beyond `PRODUCT.md`. |
| [`experience/`](experience/) | Continuous-discovery practice and user-centered evidence. |
| [`engineering/`](engineering/) | Technical lifecycle and operational documentation. |
| [`engineering/adrs/`](engineering/adrs/) | Numbered Architecture Decision Records. |

## Conventions

- Keep `PRODUCT.md` and `DESIGN.md` compact and discoverable by tools such as `impeccable`.
- Link evidence forward: experience -> requirement -> architecture/test -> release -> observation.
- Keep requirements solution-neutral; architecture owns how the system satisfies them.
- Separate configured or deployed state from verified production behavior.
- Record significant durable choices as ADRs.
- Update the relevant docs in the same change as behavior.
