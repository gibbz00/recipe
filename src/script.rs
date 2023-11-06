use std::io::Write;

use crate::recipe_directory::RecipeDirectory;

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

pub fn run(script: Script, recipe_directory: &RecipeDirectory) -> anyhow::Result<()> {
    let script_path = recipe_directory.as_ref().join(script.name());

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
