use std::sync::Arc;

use makepad_analyzer_core::config::Config;
use makepad_analyzer_session::SessionManager;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tower_lsp::Client;

const DEFAULT_SESSION_CACHE_SIZE: usize = 7;
static SESSION_MANAGER: Lazy<Arc<SessionManager>> = Lazy::new(|| {
  SessionManager::builder()
    .with_cache_capacity(DEFAULT_SESSION_CACHE_SIZE)
    .build()
});

pub struct ServerContext {
  pub(crate) client: Option<Client>,
  pub config: Arc<RwLock<Config>>,

  pub session_manager: &'static SessionManager,
}

impl Default for ServerContext {
  fn default() -> Self {
    let context = ServerContext {
      client: None,
      config: Arc::new(RwLock::new(Config::default())),
      session_manager: &*SESSION_MANAGER
    };
    context
  }
}

impl ServerContext {
  pub fn new(client: Client) -> ServerContext {
    ServerContext {
      client: Some(client),
      ..Default::default()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;


  #[tracing_test::traced_test]
  #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
  async fn test_server_context() {
    let context = ServerContext::default();
    let session_manager = context.session_manager;
    let session_manager_cache_capacity = session_manager.cache().capacity();
    tracing::info!("Session manager cache capacity: {}", session_manager_cache_capacity);
    session_manager.stop();
  }
}
