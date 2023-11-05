use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct Manifest {
    #[serde(rename = "recipe")]
    recipe_metadata: RecipeMetadata,
    #[serde(rename = "package")]
    package_metadata: PackageMetadata,
}

#[derive(Deserialize)]
struct RecipeMetadata {
    version: usize,
    maintainers: Vec<String>,
    architectures: Vec<String>,
}

#[derive(Deserialize)]
struct PackageMetadata {
    name: String,
    version: String,
    description: String,
    url: Url,
    licenses: Vec<String>,
    tags: Vec<String>,
    dependencies: PackageDependencies,
}

#[derive(Deserialize)]
struct PackageDependencies {
    build: Vec<String>,
    run: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::paths;

    use super::*;

    #[test]
    fn deserializes_template_manifest() {
        let template_manifest_path = paths::repo::template_manifest_path().unwrap();
        let manifest_template_string = std::fs::read_to_string(template_manifest_path).unwrap();
        toml::from_str::<Manifest>(&manifest_template_string).unwrap();
    }
}
