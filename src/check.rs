use crate::{manifest::Manifest, recipe_directory::RecipeDirectory};

pub fn check(recipe_directory: &RecipeDirectory) -> anyhow::Result<Manifest> {
    Manifest::from_recipe_derictory(recipe_directory)
}
