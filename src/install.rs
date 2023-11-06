use crate::{build::build_directory, recipe_directory::RecipeDirectory, script::Script};

pub fn install(recipe_directory: &RecipeDirectory) -> anyhow::Result<()> {
    let build_directory = build_directory(recipe_directory);

    if !build_directory.is_dir() {
        crate::build::build(recipe_directory)?;
    }

    std::env::set_current_dir(&build_directory)?;
    crate::script::run(Script::Install, recipe_directory)
}

#[cfg(test)]
mod tests {
    use crate::paths::repo::decoupled_recipe_directory;

    use super::*;

    #[test]
    fn installs_template() {
        decoupled_recipe_directory(install).unwrap()
    }
}
