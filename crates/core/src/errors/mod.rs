mod document_error;

pub use document_error::DocumentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MakepadAnalyzerError {
  #[error(transparent)]
  DocumentError(#[from] DocumentError)
}
