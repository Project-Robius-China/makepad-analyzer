use crate::config::Config;
use crate::handlers::request;
use std::sync::Arc;
use parking_lot::RwLock;
use tower_lsp::jsonrpc::{self, Result};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct MakepadAnalyzer {
  pub client: Option<Client>,
  pub config: Arc<RwLock<Config>>
}

impl Default for MakepadAnalyzer {
  fn default() -> Self {
      let state = MakepadAnalyzer {
          client: None,
          config: Arc::new(RwLock::new(Config::default()))
      };
      state
  }
}

impl MakepadAnalyzer {
  pub fn new(client: Client) -> MakepadAnalyzer {
    MakepadAnalyzer {
      client: Some(client),
      ..Default::default()
    }
  }

  // TODO: Implement the shutdown_analyzer method
  pub fn shutdown_analyzer(&self) -> jsonrpc::Result<()> {
    tracing::info!("Shutting down the Makepad Analyzer");
    Ok(())
  }
}

#[tower_lsp::async_trait]
impl LanguageServer for MakepadAnalyzer {
  async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
    request::handle_initialize(&self, &params)
  }

  async fn initialized(&self, _: InitializedParams) {
    tracing::info!("Makepad Analyzer Initialized");
  }

  async fn shutdown(&self) -> Result<()> {
    self.shutdown_analyzer()
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
