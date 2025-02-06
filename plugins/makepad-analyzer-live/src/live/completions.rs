use lsp_types::{CompletionItem, CompletionItemKind, Documentation, MarkupContent, MarkupKind, Position, Url};

pub fn handle_completion(
  uri: &Url,
  position: Position,
  trigger_char: &str,
) -> Vec<CompletionItem> {

  tracing::info!("completion_items: uri: {:?}, position: {:?}, trigger_char: {:?}", uri, position, trigger_char);

  // TODO: Customize the handling of completions according to the current plugin.

  let completion_items = vec![
    CompletionItem {
      label: "Some completion item".to_string(),
      kind: Some(CompletionItemKind::VARIABLE),
      detail: Some("Some detail".to_string()),
      documentation: Some(Documentation::MarkupContent(
        MarkupContent {
          kind: MarkupKind::Markdown,
          value: "Some documentation".to_string(),
        },
      )),
      ..Default::default()
    },
  ];

  completion_items
}
