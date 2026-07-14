# DocSlime

DocSlime is an opinionated CLI and agent-skill system for turning a repo into a living, agent-ready documentation workspace. It scaffolds a standardized docs tree that coding agents can fill in with you, then tighten through skills like `docslime-fill`, `docslime-adr`, and `docslime-kiss`.

The point is to keep a complete product-and-engineering learning loop in the repo: strategy, product/design context, continuous discovery, requirements, architecture, TDD/BDD evidence, delivery, observability, and decisions. DocSlime is built for services and user-facing products with user-driven Domain Driven Design and explicit traceability from evidence to production learning.

## The tree it creates

```
docs/
├── README.md              # lifecycle map and conventions
├── PRODUCT.md             # product context, audience, goals, outcomes
├── DESIGN.md              # design principles, patterns, accessibility
├── REQUIREMENTS.md        # testable contract derived from discovery
├── strategy/
│   └── README.md          # market, positioning, roadmap, strategic bets
├── experience/
│   └── README.md          # continuous discovery and experience design
└── engineering/
    ├── README.md          # engineering lifecycle index
    ├── ARCHITECTURE.md    # domain and system design
    ├── TESTING.md         # TDD/BDD strategy and CI evidence
    ├── PUBLISHING.md      # continuous delivery, verification, rollback
    ├── OBSERVABILITY.md   # production health and user-outcome learning
    └── adrs/
        └── README.md      # Architecture Decision Record index
```

`docs/PRODUCT.md` and `docs/DESIGN.md` are intentionally named so tools like `impeccable` can discover product and design context without a duplicate root-level bridge file.

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

Existing files are **never overwritten**. `init` also recognizes the previous numbered layout and will not create duplicate renamed files beside existing user work. Pass `--force` to write a current template intentionally; legacy files are still not deleted.

### Filling in the docs with an agent

After `docslime init`, point your coding agent at `docs/` and ask it to fill in a document (start with `PRODUCT.md`). Each file's `<!-- LLM: ... -->` comments tell the agent which questions to ask you and how to write the section; the agent removes those comments as it fills each part in.

## Use with an AI agent (skills)

DocSlime ships a set of [agent skills](.agents/skills) that teach Codex, Claude Code, ChatGPT, and other skill-aware tools how to drive the whole documentation lifecycle. They are plain `SKILL.md` folders with `name` / `description` frontmatter, plus `agents/openai.yaml` metadata for OpenAI/Codex-style interfaces.

Install them with the Skills CLI:

```sh
npx skills add DecisionNerd/DocSlime
```

For a specific coding agent:

```sh
npx skills add DecisionNerd/DocSlime --agent codex
npx skills add DecisionNerd/DocSlime --agent claude-code
```

ChatGPT and Claude Desktop use their own Skills UI / import flows rather than sharing local project installs automatically, but the same skill folders follow the open Agent Skills shape and can be reused there.

Then invoke them from inside your AI tool:

| Skill | What it does |
| --- | --- |
| `docslime-install` | Check the `docslime` binary is installed; install it if missing. |
| `docslime-init` | Scaffold the `docs/` tree into the current repo and orient on what to fill in. |
| `docslime-fill` | Interview you and fill in a document, following its inline `<!-- LLM: ... -->` guidance. |
| `docslime-adr` | Create the next-numbered ADR and fill it in by interviewing you about one decision. |
| `docslime-kiss` | Review docs for bloat, AI slop, weak traceability, and best-practice cleanup. |

A typical first run: `/docslime-install` → `/docslime-init` → `/docslime-fill` starting with `PRODUCT.md`, then `/docslime-kiss` once docs are filled enough to tighten.

## Development

```sh
cargo build            # debug build
cargo test             # unit + integration tests
cargo clippy --all-targets
npm run build          # build the docmd site from docs/
cargo build --release  # optimized binary at target/release/docslime
```

The templates live in [`templates/`](templates/) (the `init` tree) and [`assets/adr.md`](assets/adr.md) (the single-record ADR template). They are embedded into the binary at compile time via [`include_dir`], so editing a template requires a rebuild.

This repo is a small monorepo: the Rust CLI, the `docmd.io` site, and the bundled agent skills ship from the same checkout. `.github/workflows/ci.yml` keeps those surfaces explicit with `CLI`, `Site`, and `Agent skills` jobs, plus a branch-policy check that allows only `staging` to open pull requests to `main`; feature branches target `staging`.

## Releasing

Distribution is configured for [`cargo-dist`] in `Cargo.toml` (`[workspace.metadata.dist]`). The release workflow runs when a SemVer tag such as `v0.3.0` is pushed. It builds the macOS/Linux binaries, publishes a GitHub Release with the install script, pushes the Homebrew formula to `DecisionNerd/homebrew-tap` using `HOMEBREW_TAP_TOKEN`, and keeps the documentation content ready for publication through the `docmd.io` system.

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
