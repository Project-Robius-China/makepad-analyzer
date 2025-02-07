use std::sync::Arc;

use makepad_analyzer_core::{config::Config, errors::MakepadAnalyzerError};
use makepad_analyzer_session::Session;
use parking_lot::RwLock;
use tower_lsp::{jsonrpc, lsp_types::Url, Client};

pub struct ServerContext {
  pub(crate) client: Option<Client>,
  pub config: Arc<RwLock<Config>>,
}

impl Default for ServerContext {
  fn default() -> Self {
    let context = ServerContext {
      client: None,
      config: Arc::new(RwLock::new(Config::default()))
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

  pub async fn get_uri_and_session_from_workspace(
    &self,
    workspace_uri: &Url,
  ) -> Result<(Url, Arc<Session>), MakepadAnalyzerError> {
    let session = self.url_to_session(workspace_uri).await?;
    let uri = session.sync.workspace_to_temp_url(workspace_uri)?;
    Ok((uri, session))
  }

  async fn url_to_session(&self, uri: &Url) -> Result<Arc<Session>, MakepadAnalyzerError> {

    // Check if the session is already in the cache
    // if let Some(session) = self.session_manager.sessions.get(&uri) {
    //   return Ok(session);
    // }

    // If no session is found, create a new session
    // let session = Arc::new(Session::new());

    todo!()
  }

  pub fn shutdown_analyzer(&self) -> jsonrpc::Result<()> {
    tracing::info!("Shutting down the Makepad Analyzer");
    Ok(())
  }
}
