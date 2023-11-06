// TEMP:
#![allow(dead_code)]

use serde::Deserialize;
use url::Url;

use crate::recipe_directory::RecipeDirectory;

const MANIFEST_NAME: &str = "Recipe.toml";

#[derive(Deserialize)]
pub struct Manifest {
    #[serde(rename = "recipe")]
    recipe_metadata: RecipeMetadata,
    #[serde(rename = "package")]
    package_metadata: PackageMetadata,
}

impl Manifest {
    pub fn from_recipe_derictory(recipe_directory: &RecipeDirectory) -> anyhow::Result<Self> {
        toml::from_str::<Manifest>(&std::fs::read_to_string(recipe_directory.as_ref().join(MANIFEST_NAME))?).map_err(Into::into)
    }
}

#[derive(Deserialize)]
pub struct RecipeMetadata {
    version: usize,
    maintainers: Vec<String>,
    targets: Vec<String>,
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
    use crate::paths::{self, repo::template_recipe_directory};

    use super::*;

    #[test]
    fn template_manifest_path_exists() {
        assert!(template_recipe_directory()
            .map(|template_directory| template_directory.as_ref().join(MANIFEST_NAME))
            .unwrap()
            .is_file())
    }

    #[test]
    fn deserializes_template_manifest() {
        Manifest::from_recipe_derictory(&paths::repo::template_recipe_directory().unwrap()).unwrap();
    }
}
