use std::process::Stdio;

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

    std::process::Command::new("bash")
        .stdin(Stdio::null())
        .arg(script_path)
        .spawn()?
        .wait()?;

    Ok(())
}
