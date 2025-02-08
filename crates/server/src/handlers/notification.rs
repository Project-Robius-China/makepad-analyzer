use crate::context::ServerContext;

use makepad_analyzer_core::errors::MakepadAnalyzerError;
use tower_lsp::lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams};
use anyhow::Result;

/// Handles the `textDocument/didOpen` notification.
pub async fn handle_did_open_text_document(
  cx: &ServerContext,
  params: DidOpenTextDocumentParams
) -> Result<()> {
  tracing::info!("Opened document: {:?}", params.text_document.uri.path());
  let (uri, _session) = cx
    .uri_and_session_from_workspace(&params.text_document.uri)
    .await?;
  cx.documents.handle_open_file(&uri).await;
  Ok(())
}

/// Handles the `textDocument/didChange` notification.
pub async fn handle_did_change_text_document(
  cx: &ServerContext,
  params: DidChangeTextDocumentParams
) -> Result<(), MakepadAnalyzerError> {
  tracing::info!("Changed document: {:?}", params.text_document.uri.path());
  // if let Err(err) = cx
  // .pid_locked_files
  // .mark_file_as_dirty(&params.text_document.uri)
  // {
  //   tracing::warn!("Failed to mark file as dirty: {}", err);
  // }

  let (uri, _session) = cx
    .uri_and_session_from_workspace(&params.text_document.uri)
    .await?;
  cx
    .documents
    .write_changes_to_file(&uri, &params.content_changes)
    .await?;
  Ok(())
}

/// Handles the `textDocument/didSave` notification.
pub async fn handle_did_save_text_document(
  _cx: &ServerContext,
  params: DidSaveTextDocumentParams
) -> Result<()> {
  tracing::info!("Saved document: {:?}", params.text_document.uri.path());
  // CX
  // .pid_locked_files
  // .remove_dirty_flag(&params.text_document.uri)?;

  // session_manager.sync.resync()?
  // TODO: complile the file, and update the diagnostics

  Ok(())
}
