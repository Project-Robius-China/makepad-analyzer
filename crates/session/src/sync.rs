use std::{path::PathBuf, sync::Arc, time::SystemTime};

use dashmap::DashMap;
use makepad_analyzer_core::errors::SyncError;
use parking_lot::RwLock;
use tokio::{task::JoinHandle, time::Instant};

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
  Mainfest,
  Temp,
}

#[derive(Debug)]
pub struct SyncWorkspace {
  directories: DashMap<Directory, PathBuf>,
  notify_handle: RwLock<Option<JoinHandle<()>>>,
  sync_status: Arc<RwLock<SyncStatus>>,
  last_sync: Arc<RwLock<Instant>>,
}

impl SyncWorkspace {
  pub(crate) fn new() -> Self {
    Self {
      directories: DashMap::new(),
      notify_handle: RwLock::new(None),
      sync_status: Arc::new(RwLock::new(SyncStatus::Idle)),
      last_sync: Arc::new(RwLock::new(Instant::now())),
    }
  }
}
