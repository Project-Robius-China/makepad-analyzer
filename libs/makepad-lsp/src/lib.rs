use server::MakepadAnalyzer;
use tower_lsp::{LspService, Server};

pub mod server;
pub mod config;
pub mod handlers {
  pub mod request;
}

pub async fn start() {
    let (service, socket) = LspService::build(MakepadAnalyzer::new).finish();
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
      .serve(service)
      .await;
}
