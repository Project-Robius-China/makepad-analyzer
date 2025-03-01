use crate::context::ServerContext;

use makepad_analyzer_core::errors::MakepadAnalyzerError;
use tower_lsp::lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams};

/// Handles the `textDocument/didOpen` notification.
pub async fn handle_did_open_text_document(
  cx: &ServerContext,
  params: DidOpenTextDocumentParams
) -> Result<(), MakepadAnalyzerError> {
  tracing::info!("Opened document: {:?}", params.text_document.uri.path());

  // Get the URI and session from the workspace.
  let (uri, _session) = cx
    .session_manager
    .uri_and_session_from_workspace(&params.text_document.uri)
    .await?;

  cx.session_manager.documents.handle_open_file(&uri).await;
  Ok(())
}

/// Handles the `textDocument/didChange` notification.
pub async fn handle_did_change_text_document(
  cx: &ServerContext,
  params: DidChangeTextDocumentParams
) -> Result<(), MakepadAnalyzerError> {
  tracing::info!("Changed document: {:?}", params.text_document.uri);
  let (uri, _session) = cx
    .session_manager
    .uri_and_session_from_workspace(&params.text_document.uri)
    .await?;
  cx.session_manager.documents.write_changes_to_file(&uri, &params.content_changes).await?;
  Ok(())
}

/// Handles the `textDocument/didSave` notification.
pub async fn handle_did_save_text_document(
  cx: &ServerContext,
  params: DidSaveTextDocumentParams
) -> Result<(), MakepadAnalyzerError> {
  tracing::info!("Saved document: {:?}", params.text_document.uri);
  let (_uri, session) = cx
    .session_manager
    .uri_and_session_from_workspace(&params.text_document.uri)
    .await?;
  session.sync.resync()?;
  Ok(())
}
