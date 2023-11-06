#[cfg(test)]
pub mod repo {
    use std::{fs, path::PathBuf};

    use fs_extra::dir::CopyOptions;
    use tempfile::tempdir;

    use crate::recipe_directory::RecipeDirectory;

    fn repo_root() -> anyhow::Result<PathBuf> {
        std::env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).map_err(Into::into)
    }

    pub fn template_recipe_directory() -> anyhow::Result<RecipeDirectory> {
        repo_root()
            .map(|repo_root| repo_root.join("template"))?
            .try_into()
            .map_err(Into::into)
    }

    pub fn decoupled_recipe_directory<F>(f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&RecipeDirectory) -> anyhow::Result<()>,
    {
        let temp_template_dir = tempdir()?;

        let read_dir = fs::read_dir(template_recipe_directory()?)?;

        let mut recipe_contents = Vec::new();
        for entry_result in read_dir {
            let entry = entry_result?;
            recipe_contents.push(entry.path())
        }

        fs_extra::copy_items(&recipe_contents, &temp_template_dir, &CopyOptions::new())?;

        f(&temp_template_dir.path().try_into()?)
    }
}
