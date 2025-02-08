use std::path::PathBuf;

use lsp_types::Url;
use makepad_analyzer_core::errors::DirectoryError;

pub fn get_path_from_url(url: &Url) -> Result<PathBuf, DirectoryError> {
  url.to_file_path()
    .map_err(|()| DirectoryError::PathFromUrlFailed {
      url: url.to_string(),
    })
}
