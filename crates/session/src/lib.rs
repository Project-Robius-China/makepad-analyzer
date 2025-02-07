mod session;
mod lru_session_cache;

use std::sync::{Arc, Weak};

pub use session::*;
pub use lru_session_cache::*;
use tokio::time::{interval, Duration};

static DEFAULT_SESSION_CACHE_CAPACITY: usize = 10;      // 10 sessions
static DEFAULT_AUTO_CLEANUP_INTERVAL: u64 = 60 * 60;  // 1 hour

pub struct SessionManagerBuilder {
  // Sessions cache
  sessions_cache: LRUSessionCache,
  // Auto cleanup interval
  auto_cleanup_interval: Duration,
}

impl SessionManagerBuilder {
  pub fn new() -> Self {
    Self {
      sessions_cache: LRUSessionCache::new(DEFAULT_SESSION_CACHE_CAPACITY),
      auto_cleanup_interval: Duration::from_secs(DEFAULT_AUTO_CLEANUP_INTERVAL),
    }
  }

  pub fn set_auto_cleanup_interval(mut self, interval: Duration) -> Self {
    self.auto_cleanup_interval = interval;
    self
  }

  pub fn build(self) -> Arc<SessionManager> {
    SessionManager::new(
      self.sessions_cache,
      self.auto_cleanup_interval
    )
  }
}

pub struct SessionManager {
  // Sessions cache
  pub sessions_cache: LRUSessionCache,
  // Auto cleanup interval
  pub auto_cleanup_interval: Duration,
}

impl SessionManager {
  fn new(
    sessions_cache: LRUSessionCache,
    auto_cleanup_interval: Duration
  ) -> Arc<Self> {
    let session_manager = Arc::new(Self {
      sessions_cache,
      auto_cleanup_interval,
    });

    let weak_session_manager = Arc::downgrade(&session_manager);
    tokio::spawn(async move {
      Self::auto_cleanup_sessions_task(
        weak_session_manager,
        auto_cleanup_interval
      ).await;
    });

    session_manager
  }

  async fn auto_cleanup_sessions_task(
    weak_manager: Weak<Self>,
    auto_cleanup_interval: Duration
  ) {
    let mut interval = interval(auto_cleanup_interval);
    loop {
      interval.tick().await;
      // check if the session manager is still alive
      if let Some(manager) = weak_manager.upgrade() {
        tracing::info!("Running auto cleanup session task");
        manager.sessions_cache.cleanup_sessions();
      } else {
        break;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_session_manager_builder() {
    let session_manager = SessionManagerBuilder::new()
      .set_auto_cleanup_interval(Duration::from_secs(60))
      .build();

    assert_eq!(session_manager.auto_cleanup_interval, Duration::from_secs(60));
  }
}
