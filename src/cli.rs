use std::path::PathBuf;

use crate::manifest::Manifest;

pub trait Execute {
    fn execute(&self) -> anyhow::Result<()>;
}

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    commands: Commands,
}

impl Execute for Args {
    fn execute(&self) -> anyhow::Result<()> {
        self.commands.execute()
    }
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Check(CheckArgs),
}

impl Execute for Commands {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            Commands::Check(check_args) => check_args.execute(),
        }
    }
}

#[derive(clap::Args)]
pub struct CheckArgs {
    /// Defaults to the current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

impl Execute for CheckArgs {
    fn execute(&self) -> anyhow::Result<()> {
        let recipe_directory = self.directory.clone().unwrap_or_default();
        let _manifest = Manifest::from_recipe_derictory(&recipe_directory)?;

        Ok(())
    }
}
