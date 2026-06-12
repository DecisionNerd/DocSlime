---
name: docgen-install
description: Verifies and installs the docgen CLI. Use when setting up docgen, after installing the skill pack with npx skills, or when a docgen command is missing.
---

# docgen Install

Verify the `docgen` binary is available, and install it if it isn't.

## When to Use

- After running `npx skills add DecisionNerd/docgen` for the first time
- "Set up docgen"
- "Install docgen"
- "Is docgen installed?"
- On first use if a `docgen` command fails with "command not found"

## Steps

### 1 — Check the binary

```bash
which docgen && docgen --version
```

If it prints a path and a version, docgen is installed — you're done. Otherwise install it.

### 2 — Install

**Homebrew (recommended, macOS / Linux):**

```bash
brew install DecisionNerd/tap/docgen
```

**Install script (macOS / Linux, no Homebrew):**

```bash
curl -LsSf https://github.com/DecisionNerd/docgen/releases/latest/download/docgen-installer.sh | sh
```

**From source (any platform with Rust 1.74+):**

```bash
cargo install --git https://github.com/DecisionNerd/docgen --bins
```

### 3 — Confirm

```bash
docgen --version
docgen --help
```

You should see the version plus the `init`, `add`, and `list` subcommands.

### 4 — Report and offer next steps

Tell the user docgen is ready. If the current repo has no `docs/` tree yet, offer to run
the **docgen-init** skill to scaffold it; otherwise offer **docgen-fill** to start filling
documents in.
