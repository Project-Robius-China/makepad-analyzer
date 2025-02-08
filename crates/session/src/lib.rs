mod session;
mod lru_session_cache;

use std::sync::Arc;

use lru_session_cache::LRUSessionCache;
use tokio::{sync::Notify, time::{sleep, Duration}};


pub use session::*;

const DEFAULT_SESSION_CACHE_SIZE: usize = 7;  // 7 sessions
const DEFAULT_AUTO_CLEANUP_INTERVAL: Duration = Duration::from_secs(60 * 60);  // 1 hour

pub struct SessionManager {
  sessions_cache: LRUSessionCache,
  auto_cleanup_interval: Duration,
  stop_signal: Arc<Notify>
}

impl SessionManager {
  fn init(
    sessions_cache: LRUSessionCache,
    auto_cleanup_interval: Duration
  ) -> Arc<SessionManager> {
    let session_manager = Arc::new(SessionManager {
      sessions_cache,
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
          self.sessions_cache.cleanup_sessions().await;
        }
      }
    }
  }

  pub fn builder() -> SessionManagerBuilder {
    SessionManagerBuilder::new()
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

    assert_eq!(session_manager.sessions_cache.capacity(), 5);
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
      session_manager.sessions_cache.insert(path, session);
    }

    tracing::info!("Current cache usage: {}", session_manager.sessions_cache.current_usage()); // shuld be 1.0

    let inactived_session_path1 = PathBuf::from("session_2");
    let inactived_session_path2 = PathBuf::from("session_4");

    assert!(session_manager.sessions_cache.get(&inactived_session_path1).is_some());
    assert!(session_manager.sessions_cache.get(&inactived_session_path2).is_some());

    // Mark session inactive
    session_manager.sessions_cache.mark_session_inactive(&inactived_session_path1);

    sleep(Duration::from_secs(3)).await;

    tracing::info!("Current cache usage: {}", session_manager.sessions_cache.current_usage()); // shuld be 0.8
    assert!(session_manager.sessions_cache.get(&inactived_session_path1).is_none());

    session_manager.sessions_cache.mark_session_inactive(&inactived_session_path2);

    sleep(Duration::from_secs(3)).await;

    assert!(session_manager.sessions_cache.get(&inactived_session_path2).is_none());

    tracing::info!("Current cache usage: {}", session_manager.sessions_cache.current_usage()); // shuld be 0.6

    session_manager.stop();

  }
}
