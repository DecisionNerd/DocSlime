# docgen

A small CLI that scaffolds a standardized, **behavior-driven** documentation tree into any
git repo. The templates are written to be filled in by an AI coding agent in conversation
with you — each document carries inline `<!-- LLM: ... -->` guidance telling the agent what to
interview you about and how to write each section.

The point is to keep a project's **mission, experiences, requirements, design guidance,
architecture, and decisions in the repo itself**, so both people and coding agents have full
context locally instead of reaching for an external source.

## The tree it creates

```
docs/
├── README.md           # intro to docs and how they are organized
├── 0-MISSION.md        # the core reason why the project/repo exists
├── 1-EXPERIENCES.md    # the user experience main doc
├── 2-REQUIREMENTS.md   # what the system needs to do
├── 3-ARCHITECTURE.md   # how the system is designed
├── 4-TESTING.md        # how we test and evaluate that the system fulfills its mission
├── 0-PRODUCT/          # product, market, and mission detail beyond 0-MISSION.md
│   └── README.md
├── 1-JOURNEYS/         # user personas, user journeys, and other experience detail
│   └── README.md
├── 2-DESIGN/           # product design, style, and accessibility guidance
│   ├── README.md
│   └── style-guide.md
└── 3-ENGINEERING/      # technical documentation including testing docs
    ├── README.md
    └── ADRs/           # Architecture Decision Records
        └── README.md
```

The documents are numbered so they read as a chain: each one builds on the one before it,
from *why the project exists* down to *how we prove it works*.

## Install

### Homebrew (macOS / Linux)

```sh
brew install DecisionNerd/tap/docgen
```

### Install script (macOS / Linux)

```sh
curl -LsSf https://github.com/DecisionNerd/docgen/releases/latest/download/docgen-installer.sh | sh
```

### From source

```sh
cargo install --path .
```

## Usage

```sh
docgen init                 # scaffold the full docs/ tree into the current repo
docgen list                 # show every template and whether it already exists
docgen add 0-MISSION        # add a single document (name resolution is forgiving)
docgen add adr <slug>       # create the next-numbered ADR, e.g. 0001-<slug>.md
```

Existing files are **never overwritten** — `init` and `add` skip anything already on disk.
Pass `--force` to overwrite intentionally.

### Filling in the docs with an agent

After `docgen init`, point your coding agent at `docs/` and ask it to fill in a document
(start with `0-MISSION.md`). Each file's `<!-- LLM: ... -->` comments tell the agent which
questions to ask you and how to write the section; the agent removes those comments as it
fills each part in.

## Use with an AI agent (skills)

docgen ships a set of [agent skills](.agents/skills) that teach Codex, Claude Code, ChatGPT,
and other skill-aware tools how to drive the whole documentation lifecycle. They are plain
`SKILL.md` folders with `name` / `description` frontmatter, plus `agents/openai.yaml`
metadata for OpenAI/Codex-style interfaces.

Install them with the Skills CLI:

```sh
npx skills add DecisionNerd/docgen
```

For a specific coding agent:

```sh
npx skills add DecisionNerd/docgen --agent codex
npx skills add DecisionNerd/docgen --agent claude-code
```

ChatGPT and Claude Desktop use their own Skills UI / import flows rather than sharing local
project installs automatically, but the same skill folders follow the open Agent Skills
shape and can be reused there.

Then invoke them from inside your AI tool:

| Skill | What it does |
|---|---|
| `docgen-install` | Check the `docgen` binary is installed; install it if missing. |
| `docgen-init`    | Scaffold the `docs/` tree into the current repo and orient on what to fill in. |
| `docgen-fill`    | Interview you and fill in a document, following its inline `<!-- LLM: ... -->` guidance. |
| `docgen-adr`     | Create the next-numbered ADR and fill it in by interviewing you about one decision. |
| `docgen-kiss`    | Review docs for bloat, AI slop, weak traceability, and best-practice cleanup. |

A typical first run: `/docgen-install` → `/docgen-init` → `/docgen-fill` starting with
`0-MISSION.md`, then `/docgen-kiss` once docs are filled enough to tighten.

## Development

```sh
cargo build            # debug build
cargo test             # unit + integration tests
cargo clippy --all-targets
cargo build --release  # optimized binary at target/release/docgen
```

The templates live in [`templates/`](templates/) (the `init` tree) and
[`assets/adr.md`](assets/adr.md) (the single-record ADR template). They are embedded into the
binary at compile time via [`include_dir`], so editing a template requires a rebuild.

## Releasing

Distribution is configured for [`cargo-dist`] in `Cargo.toml` (`[workspace.metadata.dist]`).
First-time setup:

1. Create a `homebrew-tap` repo on GitHub (matches the `tap` field in `Cargo.toml`).
2. Run `dist init` to generate `.github/workflows/release.yml`.
3. Tag a release — CI builds the macOS/Linux binaries, publishes a GitHub Release with the
   install script, and pushes the Homebrew formula to the tap:

   ```sh
   git tag v0.1.0 && git push --tags
   ```

[`include_dir`]: https://crates.io/crates/include_dir
[`cargo-dist`]: https://opensource.axo.dev/cargo-dist/
