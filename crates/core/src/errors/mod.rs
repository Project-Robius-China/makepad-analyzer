mod document_error;
mod directory_error;
mod sync_error;

pub use document_error::DocumentError;
pub use sync_error::SyncError;
pub use directory_error::DirectoryError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MakepadAnalyzerError {
  #[error(transparent)]
  DocumentError(#[from] DocumentError),
  #[error(transparent)]
  SyncError(#[from] SyncError),
  #[error(transparent)]
  DirectoryError(#[from] DirectoryError),
}
