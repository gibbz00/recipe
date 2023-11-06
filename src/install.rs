use std::path::Path;

use crate::{build::build_directory, script::Script};

pub fn install(recipe_directory: &Path) -> anyhow::Result<()> {
    let build_directory = build_directory(recipe_directory);

    if !build_directory.is_dir() {
        crate::build::build(recipe_directory)?;
    }

    std::env::set_current_dir(&build_directory)?;
    crate::script::run(Script::Install, "../")
}

#[cfg(test)]
mod tests {
    use crate::paths::repo::decoupled_recipe_template;

    use super::*;

    #[test]
    fn installs_template() {
        decoupled_recipe_template(install).unwrap()
    }
}
