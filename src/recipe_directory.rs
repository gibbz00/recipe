use std::path::{Path, PathBuf};

pub struct RecipeDirectory(PathBuf);

impl TryFrom<&Path> for RecipeDirectory {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        path.canonicalize().map(Self)
    }
}

impl TryFrom<Option<PathBuf>> for RecipeDirectory {
    type Error = <Self as TryFrom<&'static Path>>::Error;

    fn try_from(optional_path_buf: Option<PathBuf>) -> Result<Self, Self::Error> {
        optional_path_buf.unwrap_or_else(|| PathBuf::from(".")).as_path().try_into()
    }
}

impl AsRef<Path> for RecipeDirectory {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn selects_current_directory_if_none() {
        assert_eq!(std::env::current_dir().unwrap(), RecipeDirectory::try_from(None).unwrap().as_ref())
    }
}
