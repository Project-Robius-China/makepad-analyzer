use makepad_analyzer_core::config::LSPClient;
use makepad_analyzer_tracing::{tracing_subscriber, FmtSpan, StdioTracingWriter};
use tower_lsp::lsp_types::{InitializeParams, InitializeResult};
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

  if let Some(workspaces) = &params.workspace_folders {
    let mut workspaces_id = 0;
    for workspace in workspaces {
      tracing::info!("Workspace {}: {:?}", workspaces_id, workspace.uri.path());
      workspaces_id += 1;
    }
    tracing::info!("Makepad Analyzer got {} workspaces", workspaces_id);
  }

  // TODO: Now we need to judge workspace whether it is Makepad project or not.

  Ok(InitializeResult {
    server_info: None,
    capabilities: capablities::server_capabilities(),
    ..InitializeResult::default()
  })
}
