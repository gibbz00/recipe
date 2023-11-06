use std::path::PathBuf;

use crate::{recipe_directory::RecipeDirectory, script::Script};

const BUILD_DIRECTORY_NAME: &str = "build";

pub fn build_directory(recipe_directory: &RecipeDirectory) -> PathBuf {
    recipe_directory.as_ref().join(BUILD_DIRECTORY_NAME)
}

pub fn build(recipe_directory: &RecipeDirectory) -> anyhow::Result<()> {
    crate::check::check(recipe_directory)?;

    let build_directory = build_directory(recipe_directory);

    if build_directory.is_dir() {
        std::fs::remove_dir_all(&build_directory)?;
    }

    std::fs::create_dir(&build_directory)?;
    std::env::set_current_dir(&build_directory)?;
    crate::script::run(Script::Build, recipe_directory)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::paths::repo::decoupled_recipe_directory;

    use super::*;

    #[test]
    fn builds_template() {
        decoupled_recipe_directory(build).unwrap()
    }
}
