---
name: docslime-install
description: Verifies and installs the DocSlime CLI. Use when setting up DocSlime, after installing the skill pack with npx skills, or when a docslime command is missing.
---

# DocSlime Install

Verify the `docslime` binary is available, and install it if it isn't.

## When to Use

- After running `npx skills add DecisionNerd/DocSlime` for the first time
- "Set up DocSlime"
- "Install DocSlime"
- "Is DocSlime installed?"
- On first use if a `docslime` command fails with "command not found"

## Steps

### 1 — Check the binary

```bash
which docslime && docslime --version
```

If it prints a path and a version, DocSlime is installed — you're done. Otherwise install it.

### 2 — Install

**Homebrew (recommended, macOS / Linux):**

```bash
brew install DecisionNerd/tap/docslime
```

**Install script (macOS / Linux, no Homebrew):**

```bash
curl -LsSf https://github.com/DecisionNerd/DocSlime/releases/latest/download/docslime-installer.sh | sh
```

**From source (any platform with Rust 1.74+):**

```bash
cargo install --git https://github.com/DecisionNerd/DocSlime --bins
```

### 3 — Confirm

```bash
docslime --version
docslime --help
```

You should see the version plus the `init`, `add`, and `list` subcommands.

### 4 — Report and offer next steps

Tell the user DocSlime is ready. If the current repo has no `docs/` tree yet, offer to run
the **docslime-init** skill to scaffold it; otherwise offer **docslime-fill** to start filling
documents in.
