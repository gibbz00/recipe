use std::path::Path;

use crate::manifest::Manifest;

pub fn check(recipe_directory: &Path) -> anyhow::Result<Manifest> {
    Manifest::from_recipe_derictory(recipe_directory)
}
