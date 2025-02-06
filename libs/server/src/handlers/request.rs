use makepad_analyzer_tracing::{tracing_subscriber::{self}, FmtSpan, StdioTracingWriter};
use tower_lsp::{jsonrpc::Result, lsp_types::{CompletionParams, CompletionResponse, Hover, HoverContents, HoverParams, InitializeParams, InitializeResult, MarkupContent, MarkupKind}};
use tracing::level_filters::LevelFilter;

use crate::{capablities, analyzer_state::MakepadAnalyzerState};

/// Request handler for the `initialize` request.
pub fn handle_initialize(
  state: &MakepadAnalyzerState,
  params: InitializeParams,
) -> Result<InitializeResult> {

  if let Some(initialization_options) = &params.initialization_options {
    let mut config = state.config.write();
    *config = serde_json::from_value(initialization_options.clone())
      .ok()
      .unwrap_or_default();
  }

  // TODO: For makepad studio, we will do special things here.

  // TODO: Start a thread that will shutdown the server if the client process is no longer active.
  // if let Some(client_pid) = params.process_id {
  //   state.spawn_client_heartbeat(client_pid as usize);
  // }

  let config = state.config.read();
  if config.logging.level != LevelFilter::OFF {
    tracing_subscriber::fmt::Subscriber::builder()
      .with_ansi(false)
      .with_max_level(LevelFilter::INFO)
      .without_time()
      .with_span_events(FmtSpan::CLOSE)
      .with_writer(StdioTracingWriter {
        writer_mode: makepad_analyzer_tracing::TracingWriterMode::Stderr,
      })
      .init();
  }

  tracing::info!("Initializing the Makepad Analyzer");

  Ok(InitializeResult {
    server_info: None,
    capabilities: capablities::server_capabilities(),
    ..InitializeResult::default()
  })
}

// TODO: Implement hover request handler
pub fn handle_hover(
  _state: &MakepadAnalyzerState,
  params: HoverParams,
) -> Result<Option<Hover>> {
  let text_document_uri = params.text_document_position_params.text_document.uri;
  tracing::info!("Hover request for: {:?}", text_document_uri);
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
pub async fn handle_completion(
  state: &MakepadAnalyzerState,
  params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
  let trigger_char = params
      .context
      .as_ref()
      .and_then(|ctx| ctx.trigger_character.as_deref())
      .unwrap_or("");
  let position = params.text_document_position.position;

  match state
        .uri_and_session_from_workspace(&params.text_document_position.text_document.uri)
        .await
  {
    Ok((uri, session)) => {
      Ok(
        session
        .completion_items(&uri, position, trigger_char)
        .map(CompletionResponse::Array)
      )
    },
    Err(err) => {
      tracing::error!("{}", err.to_string());
      Ok(None)
    }
  }
}
