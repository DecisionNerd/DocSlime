# Experiences

At its best, docgen feels like it gets out of the way. A developer runs one command, a full
documentation tree appears in the repo, and their AI agent immediately knows how to fill it
in — asking the right questions instead of guessing. The docs end up written and committed
alongside the code, and nobody had to invent a structure or fight a tool to get there.

## Primary users

- **Engineering team member** — a developer (often working with an AI coding agent) who wants
  consistent, in-repo project docs without designing a doc system from scratch. See [`1-JOURNEYS/`](1-JOURNEYS/).
- **AI coding agent** — the intended author and reader of the filled-in docs; it follows the
  inline guidance to interview the team and write each section. See [`1-JOURNEYS/`](1-JOURNEYS/).

## Key experiences

### Scaffold the docs tree

> **As a** developer starting documentation on a repo
> **I want** to drop a standardized docs tree into the project with one command
> **So that** I have a consistent structure to fill in, without inventing one

- **Given** a git repo with no `docs/` directory
- **When** I run `docgen init`
- **Then** the full numbered `docs/` tree is created, each file carrying inline `<!-- LLM: -->` guidance

Must not happen: an existing file is silently overwritten. `init` skips anything already on
disk unless `--force` is passed.

### Add a single document

> **As a** developer with a partial docs tree
> **I want** to add just one missing document by name
> **So that** I can fill gaps without re-running the whole scaffold

- **Given** a repo that already has some docs
- **When** I run `docgen add 3-ARCHITECTURE` (name matched leniently, with or without `.md`)
- **Then** only that document is created from its template, and existing files are untouched

Must not happen: an unknown name fails silently. docgen reports the error and lists the
valid template names.

### Record an architecture decision

> **As a** team making a significant technical choice
> **I want** to create the next-numbered ADR with a memorable slug
> **So that** the decision and its reasoning are captured immutably in the repo

- **Given** an ADR directory with records up to `0003-*`
- **When** I run `docgen add adr use-postgres`
- **Then** `docs/2-ENGINEERING/ADRs/0004-use-postgres.md` is created from the ADR template

Must not happen: numbering collides or restarts. docgen always picks the next number after
the highest existing record (`0001` if there are none).

### Fill in a document with an agent

> **As a** developer with a scaffolded but empty doc
> **I want** my AI agent to interview me and write the document
> **So that** the doc reflects real project intent rather than boilerplate

- **Given** a scaffolded document containing `<!-- LLM: -->` guidance comments
- **When** I ask my agent to fill it in (via the `docgen-fill` skill)
- **Then** the agent asks focused questions, writes each section, and removes the guidance
  comments as it goes — leaving a clean, completed document

Must not happen: the agent fabricates facts or leaves guidance comments behind. A finished
doc has no `LLM:` comments and no leftover placeholder prompts.

## Experience principles

- **Non-destructive by default** — existing files are never overwritten without an explicit `--force`.
- **Forgiving input** — document names and ADR slugs are matched and normalized leniently.
- **Self-explaining** — every template tells the agent (and the human) what to do; no external manual required.
- **Fast and local** — commands run instantly against the filesystem, and the output lives in the repo.
- **Small surface** — three commands (`init`, `add`, `list`) cover the whole tool.

## Out of scope

- Rendering, hosting, or publishing the docs as a website.
- Generating documentation content automatically without a human-in-the-loop interview.
- Editing or migrating docs that have already been filled in (docgen only scaffolds).
- Enforcing a single rigid format — the tree is a default, and teams may diverge.
