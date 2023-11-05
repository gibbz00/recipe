use std::path::Path;

use serde::Deserialize;
use url::Url;

const MANIFEST_NAME: &str = "Recipe.toml";

#[derive(Deserialize)]
pub struct Manifest {
    #[serde(rename = "recipe")]
    recipe_metadata: RecipeMetadata,
    #[serde(rename = "package")]
    package_metadata: PackageMetadata,
}

impl Manifest {
    pub fn from_recipe_derictory(recipe_directory: &Path) -> anyhow::Result<Self> {
        toml::from_str::<Manifest>(&std::fs::read_to_string(recipe_directory.join(MANIFEST_NAME))?).map_err(Into::into)
    }
}

#[derive(Deserialize)]
pub struct RecipeMetadata {
    version: usize,
    maintainers: Vec<String>,
    architectures: Vec<String>,
}

#[derive(Deserialize)]
pub struct PackageMetadata {
    name: String,
    version: String,
    description: String,
    url: Url,
    licenses: Vec<String>,
    tags: Vec<String>,
    dependencies: PackageDependencies,
}

#[derive(Deserialize)]
pub struct PackageDependencies {
    build: Vec<String>,
    run: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::paths::{self, repo::template_directory_path};

    use super::*;

    #[test]
    fn template_manifest_path_exists() {
        assert!(template_directory_path()
            .map(|template_directory| template_directory.join(MANIFEST_NAME))
            .unwrap()
            .is_file())
    }

    #[test]
    fn deserializes_template_manifest() {
        Manifest::from_recipe_derictory(&paths::repo::template_directory_path().unwrap()).unwrap();
    }
}
