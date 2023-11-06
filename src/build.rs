use std::path::Path;

use crate::script::Script;

const BUILD_DIRECTORY_NAME: &str = "build";

pub fn build(recipe_directory: &Path) -> anyhow::Result<()> {
    crate::check::check(recipe_directory)?;

    let build_directory = recipe_directory.join(BUILD_DIRECTORY_NAME);

    if !build_directory.is_dir() {
        std::fs::create_dir(&build_directory)?;
    }

    std::env::set_current_dir(&build_directory)?;

    crate::script::run(Script::Build, "../")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_template() {
        build(&crate::paths::repo::template_directory_path().unwrap()).unwrap()
    }
}
