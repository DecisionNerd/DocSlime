//! Command-line interface definition (clap derive).

use clap::{Parser, Subcommand};

/// Scaffold a standardized, BDD-oriented `docs/` tree into a repo.
///
/// The templates are designed to be filled in by an AI coding agent in conversation with
/// the user, keeping the project's mission, experiences, requirements, architecture, and
/// decisions in the repo itself.
#[derive(Debug, Parser)]
#[command(name = "docgen", version, about, long_about = None)]
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
    ///
    /// Examples:
    ///   docgen add 0-MISSION
    ///   docgen add 3-ARCHITECTURE.md
    ///   docgen add adr my-decision-slug
    Add {
        /// Template name (e.g. `0-MISSION`, `3-ARCHITECTURE`), or the literal `adr`.
        doc: String,

        /// When `doc` is `adr`: the short slug for the new record's filename.
        slug: Option<String>,

        /// Overwrite the file if it already exists instead of skipping it.
        #[arg(long)]
        force: bool,
    },

    /// List available templates and whether they already exist on disk.
    List,
}
