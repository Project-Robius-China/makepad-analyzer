use makepad_analyzer_core::config::LSPClient;
use makepad_analyzer_tracing::{tracing_subscriber, FmtSpan, StdioTracingWriter};
use tower_lsp::lsp_types::{CompletionParams, CompletionResponse, InitializeParams, InitializeResult};
use tracing::level_filters::LevelFilter;

use crate::{capablities, context::ServerContext};
use tower_lsp::jsonrpc::Result;

pub fn handle_initialize(
  cx: &ServerContext,
  params: InitializeParams,
) -> Result<InitializeResult> {
  if let Some(initialization_options) = &params.initialization_options {
    let mut config = cx.config.write();
    *config = serde_json::from_value(initialization_options.clone())
      .ok()
      .unwrap_or_default();
  }

  let config = cx.config.read();
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

  // Feature: For makepad studio, we will do special things here.
  if config.client == LSPClient::MakepadStudio {
    tracing::info!("Initializing the Makepad Analyzer for Makepad Studio");
  }

  Ok(InitializeResult {
    server_info: None,
    capabilities: capablities::server_capabilities(),
    ..InitializeResult::default()
  })
}

pub async fn handle_completion(
  cx: &ServerContext,
  params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
  let trigger_char = params
    .context
    .as_ref()
    .and_then(|ctx| ctx.trigger_character.as_deref())
    .unwrap_or("");
  let position = params.text_document_position.position;

  match cx
    .uri_and_session_from_workspace(&params.text_document_position.text_document.uri)
    .await
  {
    Ok((uri, session)) => {
      let text_document = cx.documents.get_text_document(&uri).unwrap();
      Ok(
        session
        .completion_items(
          &text_document,
          position,
          trigger_char
        )
        .map(CompletionResponse::Array)
      )
    }
    Err(err) => {
      tracing::error!("{}", err.to_string());
      Ok(None)
    }
  }
}
