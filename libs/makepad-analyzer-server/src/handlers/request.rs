use makepad_analyzer_tracing::{tracing_subscriber::{self}, FmtSpan, StdioTracingWriter};
use tower_lsp::{jsonrpc::Result, lsp_types::{CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult, MarkupContent, MarkupKind, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind}};
use tracing::level_filters::LevelFilter;

use crate::server::MakepadAnalyzerState;

/// Request handler for the `initialize` request.
pub fn handle_initialize(
  _state: &MakepadAnalyzerState,
  _params: InitializeParams,
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
    .without_time()
    .with_span_events(FmtSpan::CLOSE)
    .with_writer(StdioTracingWriter {
      writer_mode: makepad_analyzer_tracing::TracingWriterMode::Stderr,
  })
  .init();

  tracing::info!("Initializing the Makepad Analyzer");

  Ok(InitializeResult {
    server_info: None,
    capabilities: ServerCapabilities {
      text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
      hover_provider: Some(HoverProviderCapability::Simple(true)),
      completion_provider: Some(CompletionOptions::default()),
      ..Default::default()
    } ,
    ..InitializeResult::default()
  })
}

// TODO: Implement hover request handler
pub fn handle_hover(
  _state: &MakepadAnalyzerState,
  params: HoverParams,
) -> Result<Option<Hover>> {
  let text_document_uri = params.text_document_position_params.text_document.uri;
  tracing::info!("Hover request for: {:#?}", text_document_uri);
  Ok(Some(Hover {
    contents: HoverContents::Markup(
      MarkupContent {
        kind: MarkupKind::Markdown,
        value: [
            "# Header",
            "Some text",
            "```Rust",
            "use link::to::code;",
            "```"
        ]
        .join("\n"),
      }
    ),
    range: None
  }))
}

// TODO: Implement completion request handler
pub fn handle_completion(
  _state: &MakepadAnalyzerState,
  params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
  let trigger_character  = params
    .context
    .as_ref()
    .and_then(|ctx| ctx.trigger_character.as_deref())
    .unwrap_or("");

  let position = params.text_document_position.position;

  tracing::info!("Completion request triggered by: {:#?}, position: {:#?}", trigger_character, position);

  Ok(Some(CompletionResponse::Array(vec![
    CompletionItem::new_simple("use".to_string(), "Some detail".to_string()),
    CompletionItem::new_simple("pub".to_string(), "More detail".to_string()),
    CompletionItem::new_simple("crate::".to_string(), "More detail".to_string()),
    CompletionItem::new_simple("crate://".to_string(), "More detail".to_string())
  ])))
}
