---
type: concept
title: Publishing
description: "How verified DocSlime CLI, skill, and documentation artifacts reach users safely."
source: "https://www.docslime.dev/engineering/PUBLISHING/"
path: /engineering/PUBLISHING/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T20:50:08.950Z"
---
---
title: "Publishing"
description: "How verified DocSlime CLI, skill, and documentation artifacts reach users safely."
---

# Publishing

DocSlime publishes three related surfaces: the Rust CLI, the reusable agent-skill pack, and
the static documentation site. A build or deployment is not complete until its expected
artifact and user-facing behavior are verified.

## Artifacts and destinations

| Artifact | Destination | Version / identity |
|---|---|---|
| Rust CLI binaries and installer | GitHub Release | SemVer tag matching `Cargo.toml` |
| Homebrew formula | `DecisionNerd/homebrew-tap` | Release version and artifact checksums |
| Agent skills | GitHub repository via `npx skills add DecisionNerd/DocSlime` | Repository revision/release |
| Documentation site | Static `site/` output and configured host | Source commit/deployment |

## Continuous integration

`.github/workflows/ci.yml` validates the CLI, site, agent skills, and branch policy. Only
`staging` may open a pull request to `main`; feature branches target `staging`.

Before publishing, run:

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
npm ci
npm run build
```

## CLI and Homebrew release

Distribution is configured through `cargo-dist`. A SemVer tag runs
`.github/workflows/release.yml`, builds macOS/Linux artifacts, creates the GitHub Release,
publishes the installer, and updates the Homebrew tap.

Verify more than workflow completion:

1. Confirm the GitHub Release contains the expected version and target artifacts.
2. Confirm the Homebrew tap points at that release and its checksums.
3. Install or upgrade through Homebrew and run `docslime --version`.
4. Scaffold a throwaway repo and confirm the current template tree is emitted.

## Skill distribution

The `.agents/skills/docslime-*` directories are the canonical source. Validate them before
release, then verify a fresh `npx skills add DecisionNerd/DocSlime` install contains the same
skill set and instructions.

## Documentation site

```sh
npm run build
```

`docmd` reads `docs/` and writes the static site to `site/`. Deployment configuration is
outside the CLI; current build and runtime status must be verified through the configured
host rather than inferred from generated files alone.

## Rollback

- Revert or supersede a bad documentation/site deployment from the last known-good source
  commit and rebuild.
- For a bad CLI release, stop promotion, document the impact, and issue a corrected version;
  do not move an immutable published tag.
- For a bad Homebrew formula, restore the last known-good formula or publish the corrected
  release, then verify installation through Homebrew.

## Official references

- [cargo-dist documentation](https://opensource.axo.dev/cargo-dist/)
- [docmd Quick Start](https://docs.docmd.io/getting-started/quick-start/)
- [docmd Deployment Overview](https://docs.docmd.io/deployment/)
