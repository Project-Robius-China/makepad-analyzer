use server::MakepadAnalyzerServer;
use tower_lsp::{LspService, Server};

pub mod server;
pub mod config;
pub mod handlers;

pub async fn start() {
    let (service, socket) = LspService::new(|client| MakepadAnalyzerServer {
      client
    });
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
      .serve(service)
      .await;
}
