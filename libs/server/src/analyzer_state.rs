use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;
use makepad_analyzer_plugin_manager::{PluginManager, PLUGIN_MANAGER};
use parking_lot::RwLock;
use tower_lsp::{jsonrpc, lsp_types::Url, Client};

use crate::{config::Config, core::session::Session};

pub struct MakepadAnalyzerState {
  pub client: Option<Client>,
  pub config: Arc<RwLock<Config>>,
  pub sessions: Arc<DashMap<PathBuf, Arc<Session>>>,
  pub plugin_manager: &'static PluginManager,
}

impl Default for MakepadAnalyzerState {
  fn default() -> Self {
      let state = MakepadAnalyzerState {
        client: None,
        config: Arc::new(RwLock::new(Config::default())),
        sessions: Arc::new(DashMap::new()),
        plugin_manager: &PLUGIN_MANAGER,
      };
      state
  }
}

impl MakepadAnalyzerState {
  pub fn new(client: Client) -> MakepadAnalyzerState {
    MakepadAnalyzerState {
      client: Some(client),
      ..Default::default()
    }
  }

  pub async fn uri_and_session_from_workspace(
    &self,
    workspace_uri: &Url,
  ) -> Result<(Url, Arc<Session>)> {
    let session = self.url_to_session(workspace_uri).await?;
    let uri = workspace_uri.clone();
    Ok((uri, session))
  }

  async fn url_to_session(&self, uri: &Url) -> Result<Arc<Session>> {
    // TODO: Try to get the manifest directory from the cache
    // TODO: If the session is already in the cache, return it
    let session = Arc::new(Session::new(self.plugin_manager));
    self.sessions.insert(uri.to_file_path().unwrap(), session.clone());
    Ok(session)
  }

  // TODO: Implement the shutdown_analyzer method
  pub fn shutdown_analyzer(&self) -> jsonrpc::Result<()> {
    tracing::info!("Shutting down the Makepad Analyzer");
    Ok(())
  }
}
