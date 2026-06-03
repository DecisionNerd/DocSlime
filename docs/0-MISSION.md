# Mission

docgen scaffolds a standardized, behavior-driven documentation tree into any git repo, with
templates written to be filled in by an AI coding agent in conversation with the team. It
exists so that a project's mission, experiences, requirements, architecture, and decisions
live in the repo itself — giving both people and coding agents full local context instead of
scattering it across external tools.

## Problem

Project knowledge — why something exists, what it must do, and why past decisions were made —
tends to live everywhere except where the work happens: in wikis, chat threads, tickets, and
people's heads. Engineering teams feel this constantly, and AI coding agents feel it acutely:
an agent in the repo has the code but not the intent, so it guesses. Every team also invents
its own ad-hoc doc structure, so there's nothing consistent for an agent to read or write
against. The result is documentation that's either missing, stale, or unfindable right when
context matters most.

## Vision

docgen is the dependable, low-friction way for a team to stand up agent-ready project docs.
A team runs `docgen init`, and within minutes an agent has interviewed them and produced a
real mission, requirements, and architecture record — not empty boilerplate. The numbered
tree becomes a familiar shape across the team's repos, so anyone (human or agent) opening a
project knows exactly where the intent lives. It does this one thing well and stays small.

## Goals

- Give every repo a consistent, behavior-driven doc structure that agents can both read and write.
- Make the path from `docgen init` to a genuinely useful, filled-in document short and low-friction.
- Ensure scaffolded docs actually get completed, not abandoned as empty templates.
- Keep project intent — mission, requirements, decisions — local to the repo and version-controlled.

## Non-goals

- It is not a documentation hosting, publishing, or static-site tool — it only scaffolds and structures files.
- It does not generate documentation content on its own; an agent and the team fill the templates in.
- It is not a wiki, knowledge base, or replacement for external product/issue trackers.
- It does not lock teams into one rigid format — the tree is a sensible default, not a mandate.

## Success metrics

- **Docs get filled:** scaffolded documents are completed (no leftover `<!-- LLM: -->` guidance), not left as empty templates.
- **Adoption:** number of repos using docgen, plus Homebrew and `npx skills` installs.
- **Agent context quality:** agents give better, less speculative answers because mission, requirements, and ADRs live in the repo.
- **Low friction:** short time from `docgen init` to a first useful, filled-in document.

## Stakeholders

- **Maintainer (DecisionNerd)** — owns docgen's direction and the templates.
- **Engineering teams** — primary users; adopt the tree across their repos to standardize docs.
- **AI coding agents** — the intended authors and consumers of the filled-in docs.
