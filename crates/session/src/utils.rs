use std::path::{self, Path, PathBuf};
use anyhow::{Context, Result};
use url::Url;

pub struct MakepadManifestFile {
  manifest: MakepadManifest,
  path: PathBuf,
}

impl MakepadManifestFile {

  pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
    let dir = path.as_ref();
    tracing::info!("dir: {:?}", dir);
    let manifest_path = Self::find_cargo_toml(&dir)
    .with_context(|| format!("No Cargo.toml found in or above {:?}", dir))?;
    Self::load_from_path(manifest_path)
  }

  pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
    let file_path = path.as_ref();
    let dir = file_path.parent()
        .context("Failed to get parent directory of the file")?;
    let manifest_path = Self::find_cargo_toml(dir)
        .with_context(|| format!("No Cargo.toml found for {:?}", file_path))?;
    Self::load_from_path(manifest_path)
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  fn find_cargo_toml(start_dir: &Path) -> Option<PathBuf> {
    let mut dir = start_dir.to_path_buf();

    while dir.exists() {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Some(cargo_toml);
        }
        dir = dir.parent()?.to_path_buf();
    }
    None
  }

  fn load_from_path(manifest_path: PathBuf) -> Result<Self> {
    let manifest = MakepadManifest {
      name: "Makepad".to_string(),
    };
    Ok(Self {
      manifest,
      path: manifest_path,
    })
  }
}

pub struct MakepadManifest {
  name: String,
}

impl MakepadManifest {}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_cargo_toml() {
    let dir = Path::new("D:\\projects\\project-robius\\robrix\\src\\login\\login_screen.rs");
    let cargo_toml = MakepadManifestFile::find_cargo_toml(dir);
    assert_eq!(cargo_toml, Some(PathBuf::from("D:\\projects\\project-robius\\robrix\\Cargo.toml")));
  }
}
