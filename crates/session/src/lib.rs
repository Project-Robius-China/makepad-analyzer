mod session;
mod lru_session_cache;
mod sync;

use dashmap::DashMap;
use lsp_types::Url;
use makepad_analyzer_core::{errors::{DirectoryError, DocumentError, MakepadAnalyzerError}, manifest::MakepadManifestFile};
use makepad_analyzer_document::Documents;
pub use session::*;
pub use sync::*;

use std::{path::{Path, PathBuf}, sync::Arc};

use lru_session_cache::LRUSessionCache;
use tokio::{sync::Notify, time::{sleep, Duration}};

const DEFAULT_SESSION_CACHE_SIZE: usize = 7;  // 7 sessions
const DEFAULT_AUTO_CLEANUP_INTERVAL: Duration = Duration::from_secs(60 * 60);  // 1 hour

pub struct SessionManager {
  pub cache: LRUSessionCache,
  pub documents: Documents,
  pub manifest_cache: DashMap<Url, Arc<PathBuf>>,

  pub(crate) auto_cleanup_interval: Duration,
  pub(crate) stop_signal: Arc<Notify>
}

impl SessionManager {
  fn init(
    cache: LRUSessionCache,
    auto_cleanup_interval: Duration
  ) -> Arc<SessionManager> {
    let session_manager = Arc::new(SessionManager {
      cache,
      documents: Documents::new(),
      manifest_cache: DashMap::new(),
      auto_cleanup_interval,
      stop_signal: Arc::new(Notify::new())
    });

    // Start the auto cleanup task
    SessionManager::start_auto_cleanup_task(Arc::clone(&session_manager));

    session_manager
  }

  fn start_auto_cleanup_task(session_manager: Arc<SessionManager>) {
    tokio::spawn(async move {
      session_manager.auto_cleanup_sessions().await;
    });
  }

  async fn auto_cleanup_sessions(&self) {
    loop {
      tokio::select! {
        _ = self.stop_signal.notified() => {
          tracing::info!("Stopping the auto cleanup sessions task");
          break;
        }
        _ = sleep(self.auto_cleanup_interval) => {
          self.cache.cleanup_sessions().await;
        }
      }
    }
  }

  pub fn builder() -> SessionManagerBuilder {
    SessionManagerBuilder::new()
  }

  pub fn cache(&self) -> &LRUSessionCache {
    &self.cache
  }

  pub async fn uri_and_session_from_workspace(
    &self,
    workspace_uri: &Url,
  ) -> Result<(Url, Arc<Session>), MakepadAnalyzerError> {
    // workspace_uri = "/d%3A/projects/project-robius/robrix/src/sliding_sync.rs"

    let session = self.url_to_session(workspace_uri).await?;
    let uri = session.sync.workspace_to_temp_url(workspace_uri)?;
    Ok((uri, session))
  }

  async fn url_to_session(&self, uri: &Url) -> Result<Arc<Session>, MakepadAnalyzerError> {
    // First we need to get the manifest directory from the cache, if it exists
    let manifest_dir = if let Some(cached_manifest_dir) = self.manifest_cache.get(uri) {
      cached_manifest_dir.clone()
    } else {
      let path = PathBuf::from(uri.path());
      // To resolve the manifest directory, we need to find the nearest `Cargo.toml` file
      let manifest = MakepadManifestFile::from_dir(&path).map_err(|_| {
        DocumentError::ManifestFileNotFound {
          dir: path.to_string_lossy().to_string(),
        }
      })?;

      tracing::info!("I'm go here");

      let dir = Arc::new(
        manifest
                .path()
                .parent()
                .ok_or(DirectoryError::ManifestDirNotFound)?
                .to_path_buf(),
      );
      self.manifest_cache.insert(uri.clone(), dir.clone());
      dir
    };

    if let Some(session) = self.cache.get(&manifest_dir) {
      return Ok(session);
    }

    let session = Arc::new(Session::new());
    self.cache.insert((*manifest_dir).clone(), session.clone());

    Ok(session)
  }

  pub fn stop(&self) {
    tracing::info!("Stopping the session manager");
    self.stop_signal.notify_waiters();
  }
}

impl Drop for SessionManager {
  fn drop(&mut self) {
    self.stop();
  }
}

pub struct SessionManagerBuilder {
  cache_capacity: usize,
  auto_cleanup_interval: Duration,
}

impl SessionManagerBuilder {

  pub fn new() -> Self {
    Self {
      cache_capacity: DEFAULT_SESSION_CACHE_SIZE,
      auto_cleanup_interval: DEFAULT_AUTO_CLEANUP_INTERVAL,
    }
  }

  pub fn with_cache_capacity(mut self, capacity: usize) -> Self {
    self.cache_capacity = capacity;
    self
  }

  pub fn with_auto_cleanup_interval(mut self, interval: Duration) -> Self {
    self.auto_cleanup_interval = interval;
    self
  }

  pub fn build(self) -> Arc<SessionManager> {
    SessionManager::init(
      LRUSessionCache::new(self.cache_capacity),
      self.auto_cleanup_interval,
    )
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use super::*;

  #[tracing_test::traced_test]
  #[tokio::test(flavor = "multi_thread")]
  async fn test_session_manager_builder() {
    let session_manager = SessionManager::builder()
      .with_cache_capacity(5)
      .with_auto_cleanup_interval(Duration::from_secs(2))
      .build();

    tracing::info!("Session manager created");

    assert_eq!(session_manager.cache.capacity(), 5);
    assert_eq!(session_manager.auto_cleanup_interval, Duration::from_secs(2));

    session_manager.stop();

    tracing::info!("Session manager stopped");
  }

  #[tracing_test::traced_test]
  #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
  async fn test_session_manager_auto_cleanup() {
    let session_manager = SessionManager::builder()
      .with_cache_capacity(5)
      .with_auto_cleanup_interval(Duration::from_secs(2))
      .build();

    for i in 0..5 {
      let path = PathBuf::from(format!("session_{}", i));
      let session = Arc::new(Session::new());
      session_manager.cache.insert(path, session);
    }

    tracing::info!("Current cache usage: {}", session_manager.cache.current_usage()); // shuld be 1.0

    let inactived_session_path1 = PathBuf::from("session_2");
    let inactived_session_path2 = PathBuf::from("session_4");

    assert!(session_manager.cache.get(&inactived_session_path1).is_some());
    assert!(session_manager.cache.get(&inactived_session_path2).is_some());

    // Mark session inactive
    session_manager.cache.mark_session_inactived(&inactived_session_path1);

    sleep(Duration::from_secs(3)).await;

    tracing::info!("Current cache usage: {}", session_manager.cache.current_usage()); // shuld be 0.8
    assert!(session_manager.cache.get(&inactived_session_path1).is_none());

    session_manager.cache.mark_session_inactived(&inactived_session_path2);

    sleep(Duration::from_secs(3)).await;

    assert!(session_manager.cache.get(&inactived_session_path2).is_none());

    tracing::info!("Current cache usage: {}", session_manager.cache.current_usage()); // shuld be 0.6

    session_manager.stop();

  }
}
