use server::MakepadAnalyzerState;
use tower_lsp::{LspService, Server};

pub mod server;
pub mod config;
pub mod core;
pub mod error;
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
