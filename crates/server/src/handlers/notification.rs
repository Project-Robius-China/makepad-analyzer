use crate::context::ServerContext;

use makepad_analyzer_core::errors::MakepadAnalyzerError;
use tower_lsp::lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams};

pub async fn handle_did_open_text_document(
  cx: &ServerContext,
  params: DidOpenTextDocumentParams
) -> Result<(), MakepadAnalyzerError> {
  tracing::info!("Opened document: {:?}", params.text_document.uri);
  let (uri, session) = cx
    .session_manager
    .uri_and_session_from_workspace(&params.text_document.uri)
    .await?;
  tracing::info!("URI: {:?}", uri);
  tracing::info!("Session: {:?}", *session);
  Ok(())
}

/// Handles the `textDocument/didChange` notification.
pub async fn handle_did_change_text_document(
  _cx: &ServerContext,
  params: DidChangeTextDocumentParams
) -> Result<(), MakepadAnalyzerError> {
  tracing::info!("Changed document: {:?}", params.text_document.uri);
  Ok(())
}

/// Handles the `textDocument/didSave` notification.
pub async fn handle_did_save_text_document(
  _cx: &ServerContext,
  params: DidSaveTextDocumentParams
) -> Result<(), MakepadAnalyzerError> {
  tracing::info!("Saved document: {:?}", params.text_document.uri);
  Ok(())
}
