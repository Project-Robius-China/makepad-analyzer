use tower_lsp::lsp_types::{CompletionOptions, HoverProviderCapability, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind};

pub fn server_capabilities() -> ServerCapabilities {
  ServerCapabilities {
    text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
    hover_provider: Some(HoverProviderCapability::Simple(true)),
    completion_provider: Some(CompletionOptions::default()),
    ..Default::default()
  }
}
