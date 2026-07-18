---
name: docslime-kiss
description: Reviews a DocSlime docs tree for KISS violations, bloat, generic AI slop, weak requirements, overbuilt architecture, stale contradictions, and non-actionable design or testing docs. Use when tightening docs or applying best-practice cleanup.
---

# DocSlime KISS

Audit a DocSlime documentation tree for whether it is useful, lean, truthful, and traceable. Use this when the user asks whether docs, requirements, architecture, design guidance, ADRs, or testing docs are bloated, generic, "AI slop", overcomplicated, or not conforming to best practice.

## When to Use

- "Review these docs for bloat"
- "Run KISS on the docs"
- "Do these requirements trace to tests?"
- "Is this architecture doc honest?"
- Before merging documentation-heavy changes
- After **docslime-fill** has replaced enough scaffold guidance to judge the tree

## Core Question

Ask this of every doc tree:

```text
Are these docs helping future work, or are they bloated, generic, and full of AI slop?
```

Answer with evidence from the files, not vibes.

## Prerequisites

- A `docs/` tree should exist.
- Read the relevant docs before judging them; do not critique from filenames alone.
- If the user wants edits, work in the current repo and preserve unrelated changes.

## Guardrails

- Lead with concrete findings and file references.
- Prefer deleting filler over rewriting it into prettier filler.
- Distinguish current behavior from target architecture or future ideas.
- Do not invent missing strategy, requirements, concepts, relationships, rules, workflows, responsibility boundaries, tests, or ADRs.
- Do not penalize a project for intentionally removing irrelevant template files. Judge whether the retained tree serves its actual users, including developers and agents.
- Do not rewrite accepted ADRs except for path/index maintenance; create a superseding ADR when a decision changes.

## Workflow

1. Read `docs/README.md` and inventory the retained docs before judging; do not assume every default template should exist.
2. Identify the project type and its human and agent consumers, then check the applicable trace across experience, requirements, architecture, testing, publishing, and observation.
3. Identify bloat and slop with concrete file/line references.
4. Recommend the smallest change that makes the docs honest and useful.
5. If the user asked for edits, patch the docs directly and preserve real decisions.

## KISS Bar

Good DocSlime docs are:

- **Specific:** describe this project, not a generic software product.
- **Fit for purpose:** retain only docs that help this project's humans or agents make better decisions; omissions are explicit and authoritative external context is linked.
- **Short enough to maintain:** no filler sections kept only because the template had them.
- **Traceable:** requirements serve evidence; architecture serves requirements; tests prove behavior; publishing delivers verified artifacts; observability feeds production evidence back into discovery.
- **Terminology-aligned:** the same preferred names for meaningful concepts, relationships, rules, workflows, and lifecycle states appear across product docs, requirements, interfaces, code, and tests unless a distinction is documented.
- **Checkable:** requirements and testing claims can be verified.
- **Current-state honest:** distinguish implemented behavior from target architecture or future ideas.
- **Decision-light:** major durable choices live in ADRs; ordinary implementation detail does not.
- **Actionable:** each doc tells a future human or agent what to do differently.

## Slop Signals

Flag content when it has one or more of these smells:

- Generic praise words without concrete behavior: robust, scalable, seamless, intuitive, comprehensive, user-friendly, cutting-edge.
- Repeated ideas copied across docs without adding new information.
- Requirements that cannot be tested or that restate implementation details.
- Architecture that describes a desired future as if it already exists.
- Long lists of technologies, risks, personas, or principles with no consequence.
- Design guidance that names vibes but gives no usable rules.
- Testing docs that list commands but do not map back to behavior.
- Publishing docs that conflate built, deployed, and verified state or omit rollback.
- Observability docs that list generic logs and metrics without user outcomes, ownership, or a discovery feedback path.
- Leftover `LLM:` comments, italic placeholders, or obviously templated prose.
- Product, strategy, or design docs kept solely because the scaffold created them.
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

## Verification

```bash
grep -rn "LLM:" docs/
```

For review-only runs, verify the report cites concrete files and names the smallest useful cleanup. For edit runs, re-run the relevant search/checks and confirm the changed docs still trace product/design -> discovery -> requirements -> architecture/testing -> publishing -> observability -> discovery, with ADRs for durable choices.

## Failure Handling

- If `docs/` is missing, say DocSlime is not initialized and offer **docslime-init**.
- If docs are too incomplete to judge, report the missing inputs and suggest **docslime-fill** for the next document in the chain.
- If evidence conflicts across docs, quote the conflicting file locations and ask the user which source is current before rewriting facts.
