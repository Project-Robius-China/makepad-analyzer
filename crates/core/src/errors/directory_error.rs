use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DirectoryError {
  #[error("Can't find temporary directory")]
  TempDirNotFound,
  #[error("Can't find manifest directory")]
  ManifestDirNotFound,
  #[error("Can't extract project name from {:?}", dir)]
  CantExtractProjectName { dir: String },
  #[error("Failed to create hidden .lsp_locks directory: {0}")]
  LspLocksDirFailed(String),
  #[error("Failed to create temp directory")]
  TempDirFailed,
  #[error("Failed to canonicalize path")]
  CanonicalizeFailed,
  #[error("Failed to copy workspace contents to temp directory")]
  CopyContentsFailed,
  #[error("Failed to create build plan. {0}")]
  StripPrefixError(std::path::StripPrefixError),
  #[error("Unable to create Url from path {:?}", path)]
  UrlFromPathFailed { path: String },
  #[error("Unable to create Url from span {:?}", span)]
  UrlFromSpanFailed { span: String },
  #[error("Unable to create path from Url {:?}", url)]
  PathFromUrlFailed { url: String },
  #[error("Unable to create span from path {:?}", path)]
  SpanFromPathFailed { path: String },
  #[error("No program ID found for path {:?}", path)]
  ProgramIdNotFound { path: String },
}
