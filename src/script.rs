use std::{io::Write, path::Path};

pub const COMMON_SCRIPT_NAME: &str = "common.bash";
pub const BUILD_SCRIPT_NAME: &str = "build.bash";
pub const INSTALL_SCRIPT_NAME: &str = "install.bash";

pub enum Script {
    Build,
    Install,
}

impl Script {
    fn name(&self) -> &str {
        match self {
            Script::Build => BUILD_SCRIPT_NAME,
            Script::Install => INSTALL_SCRIPT_NAME,
        }
    }
}

pub fn run(script: Script, script_directory: impl AsRef<Path>) -> anyhow::Result<()> {
    let script_directory = script_directory.as_ref();
    exec(&script_directory.join(COMMON_SCRIPT_NAME))?;
    exec(&script_directory.join(script.name()))
}

fn exec(script_path: &Path) -> anyhow::Result<()> {
    // Intentially disallow inheriting stdin from parent.
    let output = std::process::Command::new("bash").arg(script_path).output()?;

    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;

    // TEMP: until https://doc.rust-lang.org/std/process/struct.ExitStatus.html#method.exit_ok is stabalized.
    match output.status.success() {
        true => Ok(()),
        false => anyhow::bail!("script execution failed, aborting"),
    }
}
