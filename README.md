# DocSlime

DocSlime is an opinionated CLI and agent-skill system for turning a repo into a living,
agent-ready documentation workspace. It scaffolds a standardized docs tree that coding
agents can fill in with you, then tighten through skills like `docslime-fill`,
`docslime-adr`, and `docslime-kiss`.

The point is to pull a project's product context, experiences, requirements, design
guidance, architecture, testing strategy, and ADRs into the repo itself. DocSlime is built
for services and user-facing products, with a strong bias toward TDD+BDD quality, Domain
Driven Design, explicit decisions, and docs that can publish cleanly through the `docmd.io`
system.

## The tree it creates

```
docs/
├── README.md           # intro to docs and how they are organized
├── PRODUCT.md          # product context, mission, audience, goals
├── 1-EXPERIENCES.md    # the user experience main doc
├── 2-REQUIREMENTS.md   # what the system needs to do
├── DESIGN.md           # product design, style, and accessibility guidance
├── 3-ARCHITECTURE.md   # how the system is designed
├── 4-TESTING.md        # how we test and evaluate that the system fulfills its goals
├── 0-PRODUCT/          # product, market, and positioning detail beyond PRODUCT.md
│   └── README.md
├── 1-JOURNEYS/         # user personas, user journeys, and other experience detail
│   └── README.md
└── 3-ENGINEERING/      # technical documentation including testing docs
    ├── README.md
    └── ADRs/           # Architecture Decision Records
        └── README.md
```

`docs/PRODUCT.md` and `docs/DESIGN.md` are intentionally named so tools like `impeccable`
can discover product and design context without a duplicate root-level bridge file.

## Install

### Homebrew (macOS / Linux)

```sh
brew install DecisionNerd/tap/docslime
```

### Install script (macOS / Linux)

```sh
curl -LsSf https://github.com/DecisionNerd/DocSlime/releases/latest/download/docslime-installer.sh | sh
```

### From source

```sh
cargo install --path .
```

## Usage

```sh
docslime init                 # scaffold the full docs/ tree into the current repo
docslime list                 # show every template and whether it already exists
docslime add PRODUCT          # add a single document (name resolution is forgiving)
docslime add adr <slug>       # create the next-numbered ADR, e.g. 0001-<slug>.md
```

Existing files are **never overwritten** — `init` and `add` skip anything already on disk.
Pass `--force` to overwrite intentionally.

### Filling in the docs with an agent

After `docslime init`, point your coding agent at `docs/` and ask it to fill in a document
(start with `PRODUCT.md`). Each file's `<!-- LLM: ... -->` comments tell the agent which
questions to ask you and how to write the section; the agent removes those comments as it
fills each part in.

## Use with an AI agent (skills)

DocSlime ships a set of [agent skills](.agents/skills) that teach Codex, Claude Code, ChatGPT,
and other skill-aware tools how to drive the whole documentation lifecycle. They are plain
`SKILL.md` folders with `name` / `description` frontmatter, plus `agents/openai.yaml`
metadata for OpenAI/Codex-style interfaces.

Install them with the Skills CLI:

```sh
npx skills add DecisionNerd/DocSlime
```

For a specific coding agent:

```sh
npx skills add DecisionNerd/DocSlime --agent codex
npx skills add DecisionNerd/DocSlime --agent claude-code
```

ChatGPT and Claude Desktop use their own Skills UI / import flows rather than sharing local
project installs automatically, but the same skill folders follow the open Agent Skills
shape and can be reused there.

Then invoke them from inside your AI tool:

| Skill | What it does |
|---|---|
| `docslime-install` | Check the `docslime` binary is installed; install it if missing. |
| `docslime-init`    | Scaffold the `docs/` tree into the current repo and orient on what to fill in. |
| `docslime-fill`    | Interview you and fill in a document, following its inline `<!-- LLM: ... -->` guidance. |
| `docslime-adr`     | Create the next-numbered ADR and fill it in by interviewing you about one decision. |
| `docslime-kiss`    | Review docs for bloat, AI slop, weak traceability, and best-practice cleanup. |

A typical first run: `/docslime-install` → `/docslime-init` → `/docslime-fill` starting with
`PRODUCT.md`, then `/docslime-kiss` once docs are filled enough to tighten.

## Development

```sh
cargo build            # debug build
cargo test             # unit + integration tests
cargo clippy --all-targets
cargo build --release  # optimized binary at target/release/docslime
```

The templates live in [`templates/`](templates/) (the `init` tree) and
[`assets/adr.md`](assets/adr.md) (the single-record ADR template). They are embedded into the
binary at compile time via [`include_dir`], so editing a template requires a rebuild.

## Releasing

Distribution is configured for [`cargo-dist`] in `Cargo.toml` (`[workspace.metadata.dist]`).
The release workflow runs when a SemVer tag such as `v0.3.0` is pushed. It builds the
macOS/Linux binaries, publishes a GitHub Release with the install script, pushes the
Homebrew formula to `DecisionNerd/homebrew-tap` using `HOMEBREW_TAP_TOKEN`, and keeps the
documentation content ready for publication through the `docmd.io` system.

Release checklist:

1. Update the package version in `Cargo.toml` and `Cargo.lock`.
2. Run the local gates:

   ```sh
   cargo fmt --check
   cargo test
   cargo clippy --all-targets
   ```

3. Commit the version bump, then create and push the matching tag:

   ```sh
   git tag v0.3.0
   git push origin main
   git push origin v0.3.0
   ```

[`include_dir`]: https://crates.io/crates/include_dir
[`cargo-dist`]: https://opensource.axo.dev/cargo-dist/
