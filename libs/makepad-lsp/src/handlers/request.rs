use makepad_analyzer_tracing::{tracing_subscriber, FmtSpan, StdioTracingWriter};
use serde_json::Value;
use tower_lsp::{jsonrpc::Result, lsp_types::{CompletionOptions, HoverProviderCapability, InitializeParams, InitializeResult, ServerCapabilities}};
use tracing::level_filters::LevelFilter;

use crate::server::MakepadAnalyzer;

/// Request handler for the `initialize` request.
pub fn handle_initialize(
  _state: &MakepadAnalyzer,
  _params: &InitializeParams,
) -> Result<InitializeResult> {

  // TODO: Get initialization options from the client.


  // TODO: Start a thread that will shutdown the server if the client process is no longer active.
  // if let Some(client_pid) = params.process_id {
  //   state.spawn_client_heartbeat(client_pid as usize);
  // }

  // TODO: Get level from the client configuration
  tracing_subscriber::fmt::Subscriber::builder()
    .with_ansi(false)
    .with_max_level(LevelFilter::INFO)
    .with_span_events(FmtSpan::CLOSE)
    .with_writer(StdioTracingWriter {
      writer_mode: makepad_analyzer_tracing::TracingWriterMode::Stderr,
  })
  .init();

  tracing::info!("Initializing the Makepad Analyzer");

  Ok(InitializeResult {
    server_info: None,
    capabilities: ServerCapabilities {
      hover_provider: Some(HoverProviderCapability::Simple(true)),
      completion_provider: Some(CompletionOptions::default()),
      ..Default::default()
    } ,
    ..InitializeResult::default()
  })
}
