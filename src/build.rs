use std::path::{Path, PathBuf};

use crate::script::Script;

const BUILD_DIRECTORY_NAME: &str = "build";

pub fn build_directory(recipe_directory: &Path) -> PathBuf {
    recipe_directory.join(BUILD_DIRECTORY_NAME)
}

pub fn build(recipe_directory: &Path) -> anyhow::Result<()> {
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
    use crate::paths::repo::decoupled_recipe_template;

    use super::*;

    #[test]
    fn builds_template() {
        decoupled_recipe_template(build).unwrap()
    }
}
