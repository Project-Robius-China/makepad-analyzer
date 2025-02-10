use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::errors::{DocumentError, MakepadAnalyzerError};

// Note: Because the makepad manifest file is now Cargo.toml, so now we just search for Cargo.toml,
// and load the manifest from it.

#[derive(Clone, Debug, PartialEq)]
pub struct MakepadManifestFile {
  manifest: MakepadManifest,
  path: PathBuf,
}

impl MakepadManifestFile {
  pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self, MakepadAnalyzerError> {
    let manifest_path = Self::find_cargo_toml(path.as_ref())
        .ok_or(DocumentError::ManifestFileNotFound {
            dir: path.as_ref().to_string_lossy().to_string(),
        })?;

    let manifest = MakepadManifest {
      name: "Makepad".to_string(),
    };

    Ok(Self {
      manifest,
      path: manifest_path,
    })
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  fn find_cargo_toml(start_dir: &Path) -> Option<PathBuf> {
    let mut current_dir = start_dir.to_path_buf();
    while current_dir.exists() {
      let cargo_toml = current_dir.join("Cargo.toml");
      if cargo_toml.exists() {
        return Some(cargo_toml);
      }
      current_dir = current_dir.parent()?.to_path_buf();
    }
    None
  }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct MakepadManifest {
  name: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_cargo_toml() {
    let dir = Path::new("D:\\projects\\project-robius\\robrix\\src\\login\\login_screen.rs");
    let dir2 = Path::new("E:\\makepad\\examples\\simple\\src\\app.rs");

    let cargo_toml = MakepadManifestFile::find_cargo_toml(dir);
    let cargo_toml2 = MakepadManifestFile::find_cargo_toml(dir2);
    assert_eq!(cargo_toml, Some(PathBuf::from("D:\\projects\\project-robius\\robrix\\Cargo.toml")));
    assert_eq!(cargo_toml2, Some(PathBuf::from("E:\\makepad\\examples\\simple\\Cargo.toml")));
  }
}
