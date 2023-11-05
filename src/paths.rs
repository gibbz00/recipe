pub(crate) const MANIFEST_NAME: &str = "Recipe.toml";

#[cfg(test)]
pub mod repo {
    use std::path::PathBuf;

    use super::*;

    pub fn repo_root() -> anyhow::Result<PathBuf> {
        std::env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).map_err(Into::into)
    }

    pub fn template_directory_path() -> anyhow::Result<PathBuf> {
        repo_root().map(|repo_root| repo_root.join("template"))
    }

    pub fn template_manifest_path() -> anyhow::Result<PathBuf> {
        template_directory_path().map(|template_directory| template_directory.join(MANIFEST_NAME))
    }

    #[test]
    fn gets_template_manifest() {
        assert!(template_manifest_path().unwrap().is_file())
    }
}
