use std::path::PathBuf;

use lsp_types::Url;
use makepad_analyzer_core::errors::DirectoryError;

/// Create a [Url] from a [`PathBuf`].
pub fn get_url_from_path(path: &PathBuf) -> Result<Url, DirectoryError> {
  Url::from_file_path(path).map_err(|()| DirectoryError::UrlFromPathFailed {
    path: path.to_string_lossy().to_string(),
  })
}

/// Create a [`PathBuf`] from a [Url].
pub fn get_path_from_url(url: &Url) -> Result<PathBuf, DirectoryError> {
  url.to_file_path()
    .map_err(|()| DirectoryError::PathFromUrlFailed {
      url: url.to_string(),
    })
}
