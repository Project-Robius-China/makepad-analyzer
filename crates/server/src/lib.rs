pub mod server;
pub mod context;
pub mod capablities;
pub mod handlers {
  pub mod notification;
  pub mod request;
}

use tower_lsp::{LspService, Server};
use context::ServerContext;

pub async fn start() {
  let (service, socket) =
    LspService::build(ServerContext::new).finish();

  Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
    .serve(service)
    .await;
}
