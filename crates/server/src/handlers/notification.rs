use crate::context::ServerContext;

use tower_lsp::lsp_types::{DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams};
use anyhow::Result;

/// Handles the `textDocument/didOpen` notification.
pub async fn handle_did_open_text_document(
  cx: &ServerContext,
  params: DidOpenTextDocumentParams
) -> Result<()> {
  tracing::info!("Opened document: {:?}", params.text_document.uri.path());
  Ok(())
}

/// Handles the `textDocument/didChange` notification.
pub async fn handle_did_change_text_document(
  _cx: &ServerContext,
  params: DidChangeTextDocumentParams
) -> Result<()> {
  tracing::info!("Changed document: {:?}", params.text_document.uri.path());
  Ok(())
}

/// Handles the `textDocument/didSave` notification.
pub async fn handle_did_save_text_document(
  _cx: &ServerContext,
  params: DidSaveTextDocumentParams
) -> Result<()> {
  tracing::info!("Saved document: {:?}", params.text_document.uri.path());
  Ok(())
}

/// Handles the `textDocument/didClose` notification.
pub async fn handle_did_close_text_document(
  _cx: &ServerContext,
  params: DidCloseTextDocumentParams
) -> Result<()> {
  tracing::info!("Closed document: {:?}", params.text_document.uri.path());
  Ok(())
}

