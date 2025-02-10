use std::{path::{Path, PathBuf}, sync::Arc, time::SystemTime};

use dashmap::DashMap;
use makepad_analyzer_core::errors::{DirectoryError, DocumentError, SyncError};
use makepad_analyzer_document::utils::get_url_from_path;
use parking_lot::RwLock;
use tokio::task::JoinHandle;
use lsp_types::Url;

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
  Idle,
  Syncing {
    total_files: usize,
    processed_files: usize,
  },
  Failed {
    error: Arc<SyncError>,
    timestamp: SystemTime,
  },
  Completed {
    timestamp: SystemTime,
    synced_files: usize,
  },
  Paused {
    reason: String,
    timestamp: SystemTime,
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Directory {
  Manifest,
  Temp,
}

#[derive(Debug)]
pub struct SyncWorkspace {
  directories: DashMap<Directory, PathBuf>,
  notify_handle: RwLock<Option<JoinHandle<()>>>,
  // sync_status: Arc<RwLock<SyncStatus>>,
  // last_sync: Arc<RwLock<Instant>>,
}

impl SyncWorkspace {
  pub(crate) fn new() -> Self {
    Self {
      directories: DashMap::new(),
      notify_handle: RwLock::new(None),
      // sync_status: Arc::new(RwLock::new(SyncStatus::Idle)),
      // last_sync: Arc::new(RwLock::new(Instant::now())),
    }
  }

  pub fn workspace_to_temp_url(&self, uri: &Url) -> Result<Url, DirectoryError> {
    convert_url(&uri, &self.temp_dir()?, &self.manifest_dir()?)
  }

  pub(crate) fn temp_to_workspace_url(&self, uri: &Url) -> Result<Url, DirectoryError> {
    convert_url(uri, &self.manifest_dir()?, &self.temp_dir()?)
  }

  pub(crate) fn temp_manifest_path(&self) -> Option<PathBuf> {
    self.temp_dir()
      .map(|dir| dir.join("Cargo.toml"))
      .ok()
  }

  pub fn manifest_path(&self) -> Option<PathBuf> {
    self.manifest_dir()
      .map(|dir| dir.join("Cargo.toml"))
      .ok()
  }

  pub(crate) fn manifest_dir(&self) -> Result<PathBuf, DirectoryError> {

    tracing::info!("In manifest_dir");

    self.directories
      .try_get(&Directory::Manifest)
      .try_unwrap()
      .map(|item| item.value().clone())
      .ok_or(DirectoryError::ManifestDirNotFound)
  }

  pub(crate) fn temp_dir(&self) -> Result<PathBuf, DirectoryError> {

    tracing::info!("directories: {:?}", self.directories);

    self.directories
      .try_get(&Directory::Temp)
      .try_unwrap()
      .map(|item| item.value().clone())
      .ok_or(DirectoryError::TempDirNotFound)
  }

}


fn convert_url(uri: &Url, from: &Path, to: &PathBuf) -> Result<Url, DirectoryError> {

  let a = PathBuf::from(uri.path().replace("%3A", ":").trim_start_matches("/"));

  tracing::info!("a: {:?}", a);

  let path = from.join(
    a
      .strip_prefix(to)
      .map_err(DirectoryError::StripPrefixError)?,
  );

  get_url_from_path(&path)
}
