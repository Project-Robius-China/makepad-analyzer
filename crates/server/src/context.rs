use std::sync::Arc;

use makepad_analyzer_core::{config::Config, errors::MakepadAnalyzerError};
use makepad_analyzer_document::Documents;
use makepad_analyzer_session::{Session, SessionManager};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tower_lsp::{jsonrpc, lsp_types::Url, Client};

const DEFAULT_SESSION_CACHE_SIZE: usize = 7;
static SESSION_MANAGER: Lazy<Arc<SessionManager>> = Lazy::new(|| {
  SessionManager::builder()
    .with_cache_capacity(DEFAULT_SESSION_CACHE_SIZE)
    .build()
});

pub struct ServerContext {
  pub(crate) client: Option<Client>,
  pub config: Arc<RwLock<Config>>,

  pub documents: Documents,
  pub session_manager: &'static SessionManager,
}

impl Default for ServerContext {
  fn default() -> Self {
    let context = ServerContext {
      client: None,
      config: Arc::new(RwLock::new(Config::default())),
      documents: Documents::new(),
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

  pub async fn uri_and_session_from_workspace(
    &self,
    workspace_uri: &Url,
  ) -> Result<(Url, Arc<Session>), MakepadAnalyzerError> {
    let session = self.url_to_session(workspace_uri).await?;
    // let uri = session.sync.workspace_to_temp_url(workspace_uri)?;
    // Ok((uri, session));
    todo!()
  }

  async fn url_to_session(&self, uri: &Url) -> Result<Arc<Session>, MakepadAnalyzerError> {

    // Check if the session is already in the cache
    // if let Some(session) = self.session_manager.sessions.get(&uri) {
    //   return Ok(session);
    // }

    // If no session is found, create a new session
    let session = Arc::new(Session::new());

    todo!()
  }

  pub fn shutdown_analyzer(&self) -> jsonrpc::Result<()> {
    tracing::info!("Shutting down the Makepad Analyzer");
    Ok(())
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
