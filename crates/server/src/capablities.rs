use tower_lsp::lsp_types::{ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind};

pub fn server_capabilities() -> ServerCapabilities {
  ServerCapabilities {
    text_document_sync: Some(TextDocumentSyncCapability::Kind(
      TextDocumentSyncKind::INCREMENTAL,
    )),
    ..ServerCapabilities::default()
  }
}
