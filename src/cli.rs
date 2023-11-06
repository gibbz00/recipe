use std::path::PathBuf;

use clap::Parser;

use self::{build::BuildArgs, check::CheckArgs, install::InstallArgs};

pub trait Execute {
    fn execute(self) -> anyhow::Result<()>;
}

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

impl Args {
    pub fn evaluate() -> anyhow::Result<()> {
        let args = Args::parse();
        args.command.execute()
    }
}

#[derive(clap::Subcommand)]
pub enum Command {
    Check(CheckArgs),
    /// Build recipe. Previous build files are cleared if
    /// the build directory is present.
    Build(BuildArgs),
    /// Install recipe. Attempts to build recipe if no build
    /// directory is found.
    Install(InstallArgs),
}

impl Execute for Command {
    fn execute(self) -> anyhow::Result<()> {
        match self {
            Command::Check(check_args) => check_args.execute(),
            Command::Build(build_args) => build_args.execute(),
            Command::Install(install_args) => install_args.execute(),
        }
    }
}

mod check {
    use super::*;

    #[derive(clap::Args)]
    pub struct CheckArgs {
        /// Specify recipe directory, defaults to current.
        #[arg(short, long)]
        directory: Option<PathBuf>,
    }

    impl Execute for CheckArgs {
        fn execute(self) -> anyhow::Result<()> {
            let directory = self.directory.unwrap_or_default();
            crate::check::check(&directory).map(|_| ())
        }
    }
}

mod build {
    use super::*;

    #[derive(clap::Args)]
    pub struct BuildArgs {
        /// Specify recipe directory, defaults to current.
        #[arg(short, long)]
        directory: Option<PathBuf>,
    }

    impl Execute for BuildArgs {
        fn execute(self) -> anyhow::Result<()> {
            let directory = self.directory.unwrap_or_default();
            crate::build::build(&directory)
        }
    }
}

mod install {
    use super::*;

    #[derive(clap::Args)]
    pub struct InstallArgs {
        /// Specify recipe directory, defaults to current.
        #[arg(short, long)]
        directory: Option<PathBuf>,
    }

    impl Execute for InstallArgs {
        fn execute(self) -> anyhow::Result<()> {
            let directory = self.directory.unwrap_or_default();
            crate::install::install(&directory)
        }
    }
}
