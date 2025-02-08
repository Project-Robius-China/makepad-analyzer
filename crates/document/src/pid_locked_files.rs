// use std::{fmt::Result, sync::Arc};

// use dashmap::DashMap;
// use lsp_types::Url;
// use makepad_analyzer_core::errors::{DirectoryError, DocumentError, MakepadAnalyzerError};

// use crate::utils;

// pub struct PidLockedFiles {
//   locks: DashMap<Url, Arc<PidFileLocking>>,
// }

// impl Default for PidLockedFiles {
//   fn default() -> Self {
//       Self::new()
//   }
// }

// impl PidLockedFiles {
//   pub fn new() -> Self {
//     Self {
//       locks: DashMap::new(),
//     }
//   }

//   pub fn mark_file_as_dirty(&self, uri: &Url) -> Result<(), MakepadAnalyzerError> {
//     if !self.locks.contains_key(uri) {
//       let path = utils::get_path_from_url(uri)?;
//       let file_lock = Arc::new(PidFileLocking::lsp(path));

//       file_lock
//         .lock()
//         .map_err(|e| DirectoryError::LspLocksDirFailed(e.to_string()))?;

//       self.locks.insert(uri.clone(), file_lock);
//     }
//     Ok(())
//   }

//   pub fn remove_dirty_flag(&self, uri: &Url) -> Result<(), MakepadAnalyzerError> {
//     if let Some((uri, file_lock)) = self.locks.remove(uri) {
//       file_lock
//       .release()
//       .map_err(|err| DocumentError::UnableToRemoveFile {
//           path: uri.path().to_string(),
//           err: err.to_string(),
//       })?;
//     }
//     Ok(())
//   }
// }
