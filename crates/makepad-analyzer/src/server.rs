use crate::analyzer_state::MakepadAnalyzerState;
use crate::handlers::{request, notification};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;

#[tower_lsp::async_trait]
impl LanguageServer for MakepadAnalyzerState {
  async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
    request::handle_initialize(self, params)
  }

  async fn initialized(&self, _: InitializedParams) {
    tracing::info!("Makepad Analyzer Registered Plugins: {:#?}", self.plugin_manager.get_all_registered_plugin_info());
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
