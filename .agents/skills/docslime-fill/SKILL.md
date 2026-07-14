---
name: docslime-fill
description: Fills scaffolded DocSlime documents by interviewing the user and removing inline LLM guidance. Use when writing product, discovery, requirements, design, architecture, testing, publishing, observability, or resolving placeholders.
---

# DocSlime Fill

Fill in one of the scaffolded `docs/` documents by interviewing the user and following the inline `<!-- LLM: ... -->` guidance baked into each template.

## When to Use

- "Fill in the product doc"
- "Help me write docs/PRODUCT.md"
- "Document the architecture"
- "Work through the requirements with me"
- Any time a scaffolded doc still has `<!-- LLM: ... -->` comments or `_italic prompts_` to resolve

## Prerequisites

The `docs/` tree must exist. If it doesn't, run the **docslime-init** skill first. If you only need to add one missing document, create it with `docslime add <name>` (run `docslime list` to see names).

## The document lifecycle

This is a menu and a common order, not a mandatory checklist. First identify which documents belong in this repo, then fill durable context and follow relevant evidence through delivery and production learning. Revisit earlier docs when observation changes what the team knows:

1. `PRODUCT.md` — project-level product context, when this repo owns it
2. `DESIGN.md` — reusable product or interface design rules, when applicable
3. `strategy/` — market, positioning, roadmap, and strategic bets only when owned here
4. `experience/` — continuous discovery, journeys, opportunities, hypotheses, and behavior
5. `REQUIREMENTS.md` — the testable build contract derived from product, design, and evidence
6. `engineering/ARCHITECTURE.md` — how it is designed (decisions live in `engineering/adrs/`)
7. `engineering/TESTING.md` — how tests and CI prove it before release
8. `engineering/PUBLISHING.md` — how verified artifacts reach users safely
9. `engineering/OBSERVABILITY.md` — how production evidence closes the discovery loop

## Guardrails

- Do not invent product facts, requirements, architecture, tests, decisions, users, or metrics.
- Do not fill or preserve a document merely because the template created it. Recommend removing, merging, or replacing irrelevant docs with a link to the authoritative source.
- Treat developers, operators, integrators, and coding agents as real users when they consume a service, library, SDK, CLI, or API; retain `experience/` when DX or agent experience matters.
- Ask one focused question at a time when facts are missing.
- Preserve existing user-written content unless it conflicts with a correction the user gives.
- Keep requirements testable and solution-neutral, architecture grounded in real domain boundaries, and testing mapped to Given/When/Then behavior.
- Keep publishing grounded in actual artifacts, gates, promotion, verification, and rollback.
- Present Semantic Versioning and Conventional Commits as optional publishing practices. Read the existing workflow first; never impose enforcement, rewrite history, or change release automation without explicit team agreement.
- Keep observability tied to user outcomes as well as system health; never invent telemetry.

## Steps

### 1 — Read the document and its guidance

Open the target file (e.g. `docs/PRODUCT.md`) and confirm it serves this project's actual consumers before filling it. A backend API in a large organization may reasonably omit local product strategy and visual design docs while keeping experience artifacts for developer and agent workflows. Each retained template carries two kinds of authoring cues:

- A **file-level** `<!-- LLM: ... -->` comment at the top with overall instructions and ordering for that document.
- A **section-level** `<!-- LLM: ... -->` comment under each heading listing the specific questions to ask and how to write that section. Italic prompts like `_What problem are we solving?_` are placeholders to replace with real prose.

Read all of them before asking anything, so you understand the whole document.

### 2 — Interview the user, one focused question at a time

Follow the guidance comments. Ask **one focused question at a time**, reflect the user's answer back in your own words, and confirm before writing. Don't dump every question at once and don't invent facts — if the user doesn't know something, note it and move on.

Pull context from the applicable lifecycle: when filling `REQUIREMENTS.md`, ground it in the retained local docs, experience evidence, and linked organization-level sources. When filling observability, trace signals back to requirements and discovery hypotheses.

### 3 — Write each section

Replace the italic prompt and write the real content per the section's guidance. Match the intended altitude — `PRODUCT.md` stays tight (it's product context, not a spec); `engineering/ARCHITECTURE.md` can go deeper.

### 4 — Remove the guidance comment

Once a section is written, **delete that section's `<!-- LLM: ... -->` comment**. When the whole document is done, remove the file-level comment too. A finished doc has no `LLM:` comments and no leftover italic prompts.

### 5 — Verify nothing is left

See the verification section below before reporting completion.

## Verification

```bash
grep -rn "LLM:" docs/
```

Anything still listed is unfinished. When the target doc is clean, summarize what you wrote and offer to move to the next document in the chain, record an architecture decision with the **docslime-adr** skill, or run **docslime-kiss** once enough docs exist to tighten bloat and generic AI prose.

Also check for leftover italic placeholders in the target file and confirm any requirement IDs, BDD scenarios, test references, or ADR links you added point to real docs or known gaps.

## Failure Handling

- If `docs/` does not exist, run **docslime-init** first.
- If the target document is ambiguous, ask which file to fill.
- If the user does not know an answer, record a clear open question instead of guessing.
- If filling reveals a durable product or technical decision, offer **docslime-adr** rather than burying the decision in prose.
