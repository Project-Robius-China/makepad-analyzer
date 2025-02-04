use anyhow::Result;
use lsp_types::{CompletionOptions, InitializeResult, ServerCapabilities, TextDocumentSyncKind};
use tracing::metadata::LevelFilter;
use makepad_analyzer_tracing::{tracing_subscriber, FmtSpan, StdioTracingWriter, TracingWriterMode};

pub fn handle_initialize(_params: &lsp_types::InitializeParams) -> Result<InitializeResult> {

  tracing_subscriber::fmt::Subscriber::builder()
    .with_ansi(false)
    .with_max_level(LevelFilter::OFF)
    .with_span_events(FmtSpan::CLOSE)
    .with_writer(StdioTracingWriter {
      writer_mode: TracingWriterMode::Stderr,
    })
    .init();

  tracing::info!("Initializing the Makepad Language Server");

  Ok(InitializeResult {
    server_info: None,
    capabilities: ServerCapabilities {
      completion_provider: Some(CompletionOptions::default()),
      text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
      ..Default::default()
    },
    ..InitializeResult::default()
  })
}
