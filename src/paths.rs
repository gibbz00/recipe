#[cfg(test)]
pub mod repo {
    use std::path::PathBuf;

    pub fn repo_root() -> anyhow::Result<PathBuf> {
        std::env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).map_err(Into::into)
    }

    pub fn template_directory_path() -> anyhow::Result<PathBuf> {
        repo_root().map(|repo_root| repo_root.join("template"))
    }
}
