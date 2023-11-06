use std::path::{Path, PathBuf};

pub struct RecipeDirectory(PathBuf);

impl TryFrom<&Path> for RecipeDirectory {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        path.canonicalize().map(Self)
    }
}

impl TryFrom<PathBuf> for RecipeDirectory {
    type Error = <Self as TryFrom<&'static Path>>::Error;

    fn try_from(path_buf: PathBuf) -> Result<Self, Self::Error> {
        path_buf.as_path().try_into()
    }
}

impl AsRef<Path> for RecipeDirectory {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
