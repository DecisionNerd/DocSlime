# Experiences

At its best, DocSlime feels like a project suddenly has a nervous system. A developer runs
one command, a full documentation tree appears in the repo, and their AI agent immediately
knows how to fill it in by asking the right questions instead of guessing. The docs end up
written and committed alongside the code, ready for agents, design tools, ADR review, tests,
and eventual `docmd.io` publication.

## Primary users

- **Engineering team member** — a developer (often working with an AI coding agent) who wants
  consistent, in-repo project docs without designing a doc system from scratch. See [`1-JOURNEYS/`](1-JOURNEYS/).
- **Product or design collaborator** — someone shaping a service or user-facing product who
  needs product and design context discoverable by tools like `impeccable`.
- **AI coding agent** — the intended author and reader of the filled-in docs; it follows the
  inline guidance to interview the team and write each section. See [`1-JOURNEYS/`](1-JOURNEYS/).

## Key experiences

### Scaffold the docs tree

> **As a** developer starting documentation on a repo
> **I want** to drop a standardized docs tree into the project with one command
> **So that** I have a consistent structure to fill in, without inventing one

- **Given** a git repo with no `docs/` directory
- **When** I run `docslime init`
- **Then** the full `docs/` tree is created, including `PRODUCT.md` and `DESIGN.md`, with each
  template carrying inline `<!-- LLM: -->` guidance

Must not happen: an existing file is silently overwritten. `init` skips anything already on
disk unless `--force` is passed.

### Add a single document

> **As a** developer with a partial docs tree
> **I want** to add just one missing document by name
> **So that** I can fill gaps without re-running the whole scaffold

- **Given** a repo that already has some docs
- **When** I run `docslime add 3-ARCHITECTURE` (name matched leniently, with or without `.md`)
- **Then** only that document is created from its template, and existing files are untouched

Must not happen: an unknown name fails silently. DocSlime reports the error and lists the
valid template names.

### Record an architecture decision

> **As a** team making a significant technical choice
> **I want** to create the next-numbered ADR with a memorable slug
> **So that** the decision and its reasoning are captured immutably in the repo

- **Given** an ADR directory with records up to `0003-*`
- **When** I run `docslime add adr use-postgres`
- **Then** `docs/3-ENGINEERING/ADRs/0004-use-postgres.md` is created from the ADR template

Must not happen: numbering collides or restarts. DocSlime always picks the next number after
the highest existing record (`0001` if there are none).

### Fill in a document with an agent

> **As a** developer with a scaffolded but empty doc
> **I want** my AI agent to interview me and write the document
> **So that** the doc reflects real project intent rather than boilerplate

- **Given** a scaffolded document containing `<!-- LLM: -->` guidance comments
- **When** I ask my agent to fill it in (via the `docslime-fill` skill)
- **Then** the agent asks focused questions, writes each section, and removes the guidance
  comments as it goes — leaving a clean, completed document

Must not happen: the agent fabricates facts or leaves guidance comments behind. A finished
doc has no `LLM:` comments and no leftover placeholder prompts.

### Tighten docs after filling

> **As a** developer with filled-in docs
> **I want** my agent to challenge whether they are bloated, generic, or full of AI slop
> **So that** the docs stay useful, truthful, and easy to maintain

- **Given** a filled or partially filled docslime tree
- **When** I ask the agent to run the `docslime-kiss` skill
- **Then** it gives a clear KISS verdict, flags concrete bloat with file references, and
  recommends the smallest best-practice cleanup

Must not happen: the review rewards longer docs just because they sound polished. The skill
should prefer cutting or tightening filler over rewriting it into prettier filler.

### Load product and design context in impeccable

> **As a** product or design agent
> **I want** the docs tree to expose recognizable product and design context files
> **So that** I can shape product work without asking the user to duplicate context elsewhere

- **Given** a repo initialized by DocSlime
- **When** `impeccable` resolves context
- **Then** it can load `docs/PRODUCT.md` and `docs/DESIGN.md`

Must not happen: the repo maintains separate root-level context files that drift from the
canonical docs tree.

## Experience principles

- **Non-destructive by default** — existing files are never overwritten without an explicit `--force`.
- **Forgiving input** — document names and ADR slugs are matched and normalized leniently.
- **Self-explaining** — every template tells the agent (and the human) what to do; no external manual required.
- **KISS after filling** — filled docs should be challenged for bloat, generic prose, and weak traceability.
- **Context tools fit the tree** — product/design tools should discover context from `docs/`
  instead of requiring duplicate files.
- **Publishable Markdown** — docs should stay clean enough to flow into `docmd.io`.
- **Fast and local** — commands run instantly against the filesystem, and the output lives in the repo.
- **Small surface** — three commands (`init`, `add`, `list`) cover the whole tool.

## Out of scope

- Hosting or rendering docs directly inside the CLI; publishing belongs to `docmd.io`.
- Generating documentation content automatically without a human-in-the-loop interview.
- Editing or migrating docs that have already been filled in (the CLI only scaffolds; skills
  help with review and filling).
- Enforcing a single rigid format — the tree is a default, and teams may diverge.
