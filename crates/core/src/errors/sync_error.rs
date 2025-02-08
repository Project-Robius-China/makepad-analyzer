use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SyncError {
  #[error("Sync already in progress")]
  AlreadySyncing,
}
