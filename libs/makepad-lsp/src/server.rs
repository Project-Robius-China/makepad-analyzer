
use makepad_analyzer_tracing::{tracing_subscriber, FmtSpan, StdioTracingWriter};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tracing::level_filters::LevelFilter;


#[derive(Debug)]
pub struct MakepadAnalyzerServer {
  pub client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for MakepadAnalyzerServer {
  async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {

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
      capabilities: ServerCapabilities {
          hover_provider: Some(HoverProviderCapability::Simple(true)),
          completion_provider: Some(CompletionOptions::default()),
          ..Default::default()
      },
      ..Default::default()
    })
  }

  async fn initialized(&self, _: InitializedParams) {
    tracing::info!("Makepad Analyzer Initialized");
  }

  async fn shutdown(&self) -> Result<()> {
    Ok(())
  }


  async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
    Ok(Some(CompletionResponse::Array(vec![
        CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
        CompletionItem::new_simple("Bye".to_string(), "More detail".to_string())
    ])))
}

  async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
      Ok(Some(Hover {
          contents: HoverContents::Scalar(
              MarkedString::String("You're hovering!".to_string())
          ),
          range: None
      }))
  }
}
