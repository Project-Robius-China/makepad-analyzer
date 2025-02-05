use crate::config::Config;
use crate::core::session::Session;
use crate::handlers::{request, notification};
use std::path::PathBuf;
use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;
use tower_lsp::jsonrpc::{self, Result};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct MakepadAnalyzerState {
  pub client: Option<Client>,
  pub config: Arc<RwLock<Config>>,
  pub sessions: Arc<DashMap<PathBuf, Arc<Session>>>
}

impl Default for MakepadAnalyzerState {
  fn default() -> Self {
      let state = MakepadAnalyzerState {
        client: None,
        config: Arc::new(RwLock::new(Config::default())),
        sessions: Arc::new(DashMap::new())
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
    let session = Arc::new(Session::new());
    self.sessions.insert(uri.to_file_path().unwrap(), session.clone());
    Ok(session)
  }

  // TODO: Implement the shutdown_analyzer method
  pub fn shutdown_analyzer(&self) -> jsonrpc::Result<()> {
    tracing::info!("Shutting down the Makepad Analyzer");
    Ok(())
  }
}

#[tower_lsp::async_trait]
impl LanguageServer for MakepadAnalyzerState {
  async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
    request::handle_initialize(self, params)
  }

  async fn initialized(&self, _: InitializedParams) {
    tracing::info!("Makepad Analyzer Initialized");
  }

  async fn shutdown(&self) -> Result<()> {
    self.shutdown_analyzer()
  }

  async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
    request::handle_completion(self, params).await
  }

  async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
    request::handle_hover(self, params)
  }

  async fn did_open(&self, params: DidOpenTextDocumentParams) {
    if let Err(err) = notification::handle_did_open_text_document(self, params).await {
      tracing::error!("Error handling didOpen notification: {:?}", err);
    }
  }

  async fn did_change(&self, params: DidChangeTextDocumentParams) {
    if let Err(err) = notification::handle_did_change_text_document(self, params).await {
      tracing::error!("Error handling didChange notification: {:?}", err);
    }
  }

  async fn did_save(&self, params: DidSaveTextDocumentParams) {
    if let Err(err) = notification::handle_did_save_text_document(self, params).await {
      tracing::error!("Error handling didSave notification: {:?}", err);
    }
  }

  async fn did_close(&self, params: DidCloseTextDocumentParams) {
    if let Err(err) = notification::handle_did_close_text_document(self, params).await {
      tracing::error!("Error handling didClose notification: {:?}", err);
    }
  }
}
