use anyhow::Result;
use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types::{request::Completion, CompletionOptions, InitializeParams, ServerCapabilities, TextDocumentSyncKind, Url};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
  eprintln!("Starting Makepad LSP server...");

  let (connection, io_threads) = Connection::stdio();
  let server_capabilities = serde_json::to_value(&ServerCapabilities {
    completion_provider: Some(CompletionOptions::default()),
    text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
    ..Default::default()
  }).unwrap();

  eprintln!("Wating for initialize.....");

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

  if let Some(workspace_folders) = init_params.workspace_folders {
      for folder in workspace_folders {
        scan_workspace(folder.uri).await?;
      }
  }

  eprintln!("Initialized");

  main_loop(connection)?;
  io_threads.join()?;

  Ok(())
}

async fn scan_workspace(uri: Url) -> Result<bool> {
  eprintln!("Scanning workspace: {}", uri);
  Ok(true)
}

fn main_loop(connection: Connection) -> Result<()> {

  for msg in &connection.receiver {
    match msg {
      Message::Request(req) => {
        if connection.handle_shutdown(&req)? {
          return Ok(());
        }
        match cast::<Completion>(req) {
          Ok((id, _params)) => {
            connection.sender.send(Message::Response(
                Response{
                    id,
                    result: Some(Value::String("Hello, World!".to_string())),
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
