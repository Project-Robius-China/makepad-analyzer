use tower_lsp::{jsonrpc::Result, lsp_types::{CompletionParams, CompletionResponse, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeParams, InitializeResult, InitializedParams}, LanguageServer};

use crate::{context::ServerContext, handlers::{notification, request}};

#[tower_lsp::async_trait]
impl LanguageServer for ServerContext {
  async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
    request::handle_initialize(&self, params)
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
    tracing::info!("Closed document: {:?}", params.text_document.uri.path());
  }
}
