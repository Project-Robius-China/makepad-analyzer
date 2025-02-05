use lsp_types::{CompletionItem, CompletionItemKind, Documentation, MarkupContent, MarkupKind, Position, Url};

pub fn completion_items(
  _uri: &Url,
  _position: Position,
  _trigger_char: &str,
) -> Option<Vec<CompletionItem>> {

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

  Some(completion_items)
}
