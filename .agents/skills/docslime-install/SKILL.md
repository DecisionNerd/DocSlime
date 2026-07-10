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

## Prerequisites

- Run from the target repo when possible so the follow-up recommendation can inspect whether
  `docs/` already exists.
- Prefer Homebrew on macOS/Linux when available; use the install script or source install as
  fallbacks.

## Guardrails

- Install the `docslime` binary; do not describe `docslime-kiss` as a CLI command.
- Do not run `docslime init` from this skill unless the user explicitly asks for setup beyond
  installation. Offer **docslime-init** as the next step instead.
- If Homebrew reports a tap or trust issue, diagnose that path before switching installers.

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

## Verification

- `which docslime` prints an executable path.
- `docslime --version` prints a version.
- `docslime --help` lists `init`, `add`, and `list`, and does not list `kiss`.

## Failure Handling

- If `docslime` is still missing, report the attempted install path and the exact failure.
- If Homebrew is present but the tap cannot be read, ask the user before changing taps or
  trust settings.
- If no installer can run in the current environment, leave the repo untouched and give the
  user the shortest manual install command that applies.

### 4 — Report and offer next steps

Tell the user DocSlime is ready. If the current repo has no `docs/` tree yet, offer to run
the **docslime-init** skill to scaffold it; otherwise offer **docslime-fill** to start filling
documents in.
