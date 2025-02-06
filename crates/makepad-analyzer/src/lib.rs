use tower_lsp::{LspService, Server};

pub mod server;
pub mod analyzer_state;
pub mod config;
pub mod capablities;
pub mod core;
pub mod error;

use analyzer_state::*;

pub mod handlers {
  pub mod request;
  pub mod notification;
}

pub async fn start() {
  let (service, socket) = LspService::build(MakepadAnalyzerState::new).finish();
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
      .serve(service)
      .await;
}
