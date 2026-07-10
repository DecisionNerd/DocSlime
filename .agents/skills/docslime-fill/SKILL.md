---
name: docslime-fill
description: Fills scaffolded DocSlime documents by interviewing the user and removing inline LLM guidance. Use when writing product, experiences, requirements, design, architecture, testing docs, or resolving placeholders.
---

# DocSlime Fill

Fill in one of the scaffolded `docs/` documents by interviewing the user and following the
inline `<!-- LLM: ... -->` guidance baked into each template.

## When to Use

- "Fill in the product doc"
- "Help me write docs/PRODUCT.md"
- "Document the architecture"
- "Work through the requirements with me"
- Any time a scaffolded doc still has `<!-- LLM: ... -->` comments or `_italic prompts_` to resolve

## Prerequisites

The `docs/` tree must exist. If it doesn't, run the **docslime-init** skill first. If you only
need to add one missing document, create it with `docslime add <name>` (run `docslime list` to
see names).

## The document chain

The top-level docs build on each other — fill them roughly in order, because later ones
reference earlier ones:

1. `PRODUCT.md` — what the product is, who it serves, and why it exists
2. `1-EXPERIENCES.md` — the user experience (detail in `1-JOURNEYS/`)
3. `2-REQUIREMENTS.md` — what the system must do
4. `3-ARCHITECTURE.md` — how it's designed (decisions captured as ADRs — see **docslime-adr**)
5. `4-TESTING.md` — how we prove it fulfills the product goals
6. `publishing.md` — how clean Markdown hands off to a docs publishing system

`0-PRODUCT/`, `1-JOURNEYS/`, `DESIGN.md`, and `3-ENGINEERING/` hold deeper detail beyond
the top-level docs.

## Guardrails

- Do not invent product facts, requirements, architecture, tests, decisions, users, or metrics.
- Ask one focused question at a time when facts are missing.
- Preserve existing user-written content unless it conflicts with a correction the user gives.
- Keep requirements testable, architecture grounded in real domain boundaries, and testing
  docs mapped to Given/When/Then behavior.
- Keep publishing guidance thin: link to official publishing docs rather than copying platform
  instructions.

## Steps

### 1 — Read the document and its guidance

Open the target file (e.g. `docs/PRODUCT.md`). Each template carries two kinds of
authoring cues:

- A **file-level** `<!-- LLM: ... -->` comment at the top with overall instructions and
  ordering for that document.
- A **section-level** `<!-- LLM: ... -->` comment under each heading listing the specific
  questions to ask and how to write that section. Italic prompts like `_What problem are we
  solving?_` are placeholders to replace with real prose.

Read all of them before asking anything, so you understand the whole document.

### 2 — Interview the user, one focused question at a time

Follow the guidance comments. Ask **one focused question at a time**, reflect the user's
answer back in your own words, and confirm before writing. Don't dump every question at
once and don't invent facts — if the user doesn't know something, note it and move on.

Pull context from earlier docs in the chain: when filling `2-REQUIREMENTS.md`, ground it in
what `PRODUCT.md` and `1-EXPERIENCES.md` already say.

### 3 — Write each section

Replace the italic prompt and write the real content per the section's guidance. Match the
intended altitude — `PRODUCT.md` stays tight (it's product context, not a spec);
`3-ARCHITECTURE.md` can go deeper.

### 4 — Remove the guidance comment

Once a section is written, **delete that section's `<!-- LLM: ... -->` comment**. When the
whole document is done, remove the file-level comment too. A finished doc has no `LLM:`
comments and no leftover italic prompts.

### 5 — Verify nothing is left

See the verification section below before reporting completion.

## Verification

```bash
grep -rn "LLM:" docs/
```

Anything still listed is unfinished. When the target doc is clean, summarize what you wrote
and offer to move to the next document in the chain, record an architecture decision with
the **docslime-adr** skill, or run **docslime-kiss** once enough docs exist to tighten bloat and
generic AI prose.

Also check for leftover italic placeholders in the target file and confirm any requirement
IDs, BDD scenarios, test references, or ADR links you added point to real docs or known gaps.

## Failure Handling

- If `docs/` does not exist, run **docslime-init** first.
- If the target document is ambiguous, ask which file to fill.
- If the user does not know an answer, record a clear open question instead of guessing.
- If filling reveals a durable product or technical decision, offer **docslime-adr** rather
  than burying the decision in prose.
