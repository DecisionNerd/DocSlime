//! `docslime` — scaffold a standardized, BDD-oriented `docs/` tree into a repo.

mod cli;
mod commands;
mod scaffold;
mod templates;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    // All commands operate relative to the current working directory.
    let root: PathBuf = std::env::current_dir()?;

    match cli.command {
        Command::Init { force } => commands::init::run(&root, force),
        Command::Add { doc, slug, force } => {
            commands::add::run(&root, &doc, slug.as_deref(), force)
        }
        Command::List => commands::list::run(&root),
    }
}
