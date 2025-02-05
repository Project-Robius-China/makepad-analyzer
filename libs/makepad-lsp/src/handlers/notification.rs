
// Client support for textDocument/didOpen, textDocument/didChange and textDocument/didClose
// notifications is mandatory in the protocol and clients can not opt out supporting them.
// See more info: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_synchronization

use tower_lsp::lsp_types::{DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams};
use anyhow::Result;
use crate::server::MakepadAnalyzerState;

pub async fn handle_did_open_text_document(
  _state: &MakepadAnalyzerState,
  params: DidOpenTextDocumentParams
) -> Result<()> {
  tracing::info!("Opened document: {:?}", params.text_document.uri.path());
  Ok(())
}

pub async fn handle_did_change_text_document(
  _state: &MakepadAnalyzerState,
  params: DidChangeTextDocumentParams
) -> Result<()> {
  tracing::info!("Changed document: {:?}", params.text_document.uri.path());
  Ok(())
}

pub async fn handle_did_save_text_document(
  _state: &MakepadAnalyzerState,
  params: DidSaveTextDocumentParams
) -> Result<()> {
  tracing::info!("Saved document: {:?}", params.text_document.uri.path());
  Ok(())
}

pub async fn handle_did_close_text_document(
  _state: &MakepadAnalyzerState,
  params: DidCloseTextDocumentParams
) -> Result<()> {
  tracing::info!("Closed document: {:?}", params.text_document.uri.path());
  Ok(())
}

