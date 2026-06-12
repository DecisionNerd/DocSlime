---
name: docgen-kiss
description: Reviews a docgen docs tree for KISS violations, bloat, generic AI slop, weak requirements, overbuilt architecture, stale contradictions, and non-actionable design or testing docs. Use when tightening docs or applying best-practice cleanup.
---

# docgen KISS

Audit a docgen documentation tree for whether it is useful, lean, truthful, and traceable.
Use this when the user asks whether docs, requirements, architecture, design guidance, ADRs,
or testing docs are bloated, generic, "AI slop", overcomplicated, or not conforming to best
practice.

## Core Question

Ask this of every doc tree:

```text
Are these docs helping future work, or are they bloated, generic, and full of AI slop?
```

Answer with evidence from the files, not vibes.

## Workflow

1. Read the docs tree before judging: `docs/README.md`, `0-MISSION.md`,
   `1-EXPERIENCES.md`, `2-REQUIREMENTS.md`, `2-DESIGN/`, `3-ARCHITECTURE.md`,
   `3-ENGINEERING/`, `4-TESTING.md`, and relevant ADRs.
2. Check the trace: mission -> experiences -> requirements -> design/architecture -> tests.
3. Identify bloat and slop with concrete file/line references.
4. Recommend the smallest change that makes the docs honest and useful.
5. If the user asked for edits, patch the docs directly and preserve real decisions.

## KISS Bar

Good docgen docs are:

- **Specific:** describe this project, not a generic software product.
- **Short enough to maintain:** no filler sections kept only because the template had them.
- **Traceable:** requirements serve experiences; architecture serves requirements; tests prove behavior.
- **Checkable:** requirements and testing claims can be verified.
- **Current-state honest:** distinguish implemented behavior from target architecture or future ideas.
- **Decision-light:** major durable choices live in ADRs; ordinary implementation detail does not.
- **Actionable:** each doc tells a future human or agent what to do differently.

## Slop Signals

Flag content when it has one or more of these smells:

- Generic praise words without concrete behavior: robust, scalable, seamless, intuitive,
  comprehensive, user-friendly, cutting-edge.
- Repeated ideas copied across docs without adding new information.
- Requirements that cannot be tested or that restate implementation details.
- Architecture that describes a desired future as if it already exists.
- Long lists of technologies, risks, personas, or principles with no consequence.
- Design guidance that names vibes but gives no usable rules.
- Testing docs that list commands but do not map back to behavior.
- Leftover `LLM:` comments, italic placeholders, or obviously templated prose.
- ADRs that relitigate the whole system instead of one decision.

## Review Output

Lead with a clear verdict:

```text
Verdict: KISS pass / needs tightening / bloated / misleading
```

Then report:

- **Findings:** concrete issues by severity, with file/line references.
- **Keep:** docs or sections that are already crisp.
- **Cut or tighten:** sections to delete, shorten, split, or rewrite.
- **Best-practice shape:** the target structure after cleanup.
- **Patch plan:** only if edits are requested or clearly implied.

## Editing Rules

- Prefer deleting filler over rewriting it into prettier filler.
- Keep the user's exact product facts and corrections.
- Do not invent missing strategy, architecture, requirements, test coverage, or design sources.
- Do not rewrite accepted ADRs except for path/index maintenance; create a superseding ADR when the decision changes.
- Keep generated docs plain Markdown with direct headings, compact tables, and no decorative prose.
