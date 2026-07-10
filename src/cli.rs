//! Command-line interface definition (clap derive).

use clap::{Parser, Subcommand};

/// Scaffold a standardized, BDD-oriented `docs/` tree into a repo.
///
/// The templates are designed to be filled in by an AI coding agent in conversation with
/// the user, keeping the project's product context, experiences, requirements, design guidance,
/// architecture, and decisions in the repo itself.
#[derive(Debug, Parser)]
#[command(name = "docslime", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Scaffold the full docs/ tree into the current directory.
    Init {
        /// Overwrite files that already exist instead of skipping them.
        #[arg(long)]
        force: bool,
    },

    /// Add a single document, or create the next-numbered ADR.
    #[command(long_about = "\
Add a single document, or create the next-numbered ADR.

Two modes, chosen by the DOC argument:

  Single template — pass a template name to scaffold just that one
  document (the name is matched leniently, with or without the .md
  extension). Run `docslime list` to see every name.

      docslime add PRODUCT
      docslime add ARCHITECTURE

  ADR — pass the literal `adr` plus a slug to create an Architecture
  Decision Record. The file is written to docs/engineering/adrs/ as
  NNNN-<slug>.md, where NNNN is the next number after the highest
  existing record (0001 if there are none yet). The slug is lower-cased
  and hyphenated, so `\"My First Decision\"` becomes `my-first-decision`.

      docslime add adr my-decision-slug    -> 0001-my-decision-slug.md
      docslime add adr \"Use Postgres\"      -> 0002-use-postgres.md

Existing files are skipped unless --force is given.")]
    Add {
        /// Template name (e.g. `PRODUCT`, `ARCHITECTURE`), or the literal `adr`.
        doc: String,

        /// Slug for the new ADR's filename (only used with `add adr <slug>`).
        ///
        /// Lower-cased and hyphenated automatically; ignored for non-ADR templates.
        slug: Option<String>,

        /// Overwrite the file if it already exists instead of skipping it.
        #[arg(long)]
        force: bool,
    },

    /// List available templates and whether they already exist on disk.
    List,
}
