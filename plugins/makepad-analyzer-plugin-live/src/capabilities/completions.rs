use lsp_types::{CompletionItem, CompletionItemKind, Documentation, InsertTextFormat, MarkupContent, MarkupKind, Position, Url};

enum LiveCompletionContext {
  Link,     // link::
  Crate,    // crate::
  Use,      // use
  None,     // pub and dep, etc. No need to dynamically generate completion items.
}

pub fn handle_completion(
  uri: &Url,
  position: Position,
  trigger_char: &str,
) -> Vec<CompletionItem> {

  let shifted_position = Position {
    line: position.line,
    character: position.character - trigger_char.len() as u32 - 1,
  };

  tracing::info!("completion_items: shifted_position: {:?}", shifted_position);

  // let document = match session.get_document(uri) {
  //   Some(document) => document,
  //   None => return None,
  // };

  // let line = match document.get_line(position.line as usize) {
  //   Some(line) => line,
  //   None => return None,
  // };

  // match match_keyword(line, position.character as usize, trigger_char) {
  //   LiveCompletionContext::Link => completion_items_for_link_keyword(),
  //   LiveCompletionContext::Crate => completion_items_for_crate_keyword(),
  //   LiveCompletionContext::Use => completion_items_for_use_keyword(),
  //   LiveCompletionContext::None => completion_items_for_static_keywords()
  // };

  let static_completion_itmes = completion_items_for_static_keywords();

  let mut completion_items = vec![];

  completion_items.extend(static_completion_itmes);

  completion_items
}

fn match_keyword(line: &str, character: usize, trigger_char: &str) -> LiveCompletionContext {
  todo!()
}

/// Returns completion items for the `link` keyword.
fn completion_items_for_link_keyword() -> Vec<CompletionItem> {
  todo!()
}

/// Returns completion items for the `crate::` and `crate://` keywords.
fn completion_items_for_crate_keyword() -> Vec<CompletionItem> {
  todo!()
}

/// Returns completion items for the `use` keyword.
fn completion_items_for_use_keyword() -> Vec<CompletionItem> {
  vec![
    CompletionItem {
      label: "use".to_string(),
      kind: Some(CompletionItemKind::KEYWORD),
      documentation: Some(Documentation::MarkupContent(
        MarkupContent {
          kind: MarkupKind::Markdown,
          value: ["# use",
            "The `use` keyword is used to import modules.",
            "Example: ",
            "```rust",
            "use link::theme::*;",
            "use link::shaders::*;",
            "use link::widgets::*;",
            " ",
            "use crate::shared::styles::*;",
            "use crate::shared::helpers::*;",
            "```",
          ].join("\n"),
        }
      )),
      ..Default::default()
    }
  ]
}

/// Returns static completion items for keywords the `pub` and `dep("")`.
fn completion_items_for_static_keywords() -> Vec<CompletionItem> {
  vec![
    CompletionItem {
      label: "pub".to_string(),
      kind: Some(CompletionItemKind::KEYWORD),
      documentation: Some(Documentation::MarkupContent(
        MarkupContent {
          kind: MarkupKind::Markdown,
          value: ["# pub",
            "The `pub` keyword is used to export the widget.",
            "Example: ",
            "```rust",
            "pub Widget = {{Widget}} {}",
            "```",
          ].join("\n"),
        }
      )),
      ..Default::default()
    },
    CompletionItem {
      label: "dep".to_string(),
      kind: Some(CompletionItemKind::KEYWORD),
      documentation: Some(Documentation::MarkupContent(
        MarkupContent {
            kind: MarkupKind::Markdown,
            value: [
              "**dep**",
              "The `dep` keyword is used to declare a dependency.",
              "Example: ",
              "```rust",
              "ICON_ADD = dep(\"crate://self/resources/icon_add.svg\")",
              "```",
            ]
            .join("\n"),
        },
      )),
      insert_text: Some("dep(\"$1\")".to_string()),
      insert_text_format: Some(InsertTextFormat::SNIPPET),
      ..Default::default()
    }
  ]
}
