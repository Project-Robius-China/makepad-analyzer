use anyhow::Result;
use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types::{request::Completion, CompletionItem, CompletionItemKind, CompletionOptions, CompletionResponse, InitializeParams, ServerCapabilities, TextDocumentSyncKind};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
  eprintln!("Starting Makepad LSP Server ...");

  let (connection, io_threads) = Connection::stdio();
  let server_capabilities = serde_json::to_value(&ServerCapabilities {
    completion_provider: Some(CompletionOptions::default()),
    text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
    ..Default::default()
  }).unwrap();

  eprintln!("Wating for initialize ...");

  let initialization_params = match connection.initialize(server_capabilities) {
    Ok(it) => it,
    Err(e) => {
      if e.channel_is_disconnected() {
          io_threads.join()?;
      }
      return Err(e.into());
    }
  };

  let init_params: InitializeParams = serde_json::from_value(initialization_params)?;

  eprintln!("Got initialize request: {:#?}", init_params);

  main_loop(connection)?;
  io_threads.join()?;

  Ok(())
}

fn main_loop(connection: Connection) -> Result<()> {

  for msg in &connection.receiver {
    match msg {
      Message::Request(req) => {
        if connection.handle_shutdown(&req)? {
          return Ok(());
        }

        // Completion request
        match cast::<Completion>(req) {
          Ok((id, params)) => {
            let res = CompletionResponse::Array(vec![
              CompletionItem {
                label: "Hello".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some("Hello, World!".to_string()),
                ..Default::default()
              },
              CompletionItem {
                label: "Bye".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some("Goodbye".to_string()),
                ..Default::default()
              },
            ]);

            eprintln!("sending completion response: {res:#?}");

            connection.sender.send(Message::Response(
              Response {
                id,
                result: Some(serde_json::to_value(&res).unwrap_or_default()),
                error: None,
              }
            ))?;

            continue;
          }
          Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
          Err(ExtractError::MethodMismatch(req)) => req,
        };
      }
      Message::Response(resp) => {
        eprintln!("got response: {resp:#?}");
      }
      Message::Notification(_) => {}
    }
  }

  Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
