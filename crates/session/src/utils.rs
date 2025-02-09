use std::vec;

use lsp_types::{CompletionItem, CompletionItemKind, Documentation, InsertTextFormat, MarkupContent, MarkupKind, Position, Url};
use makepad_analyzer_document::TextDocument;

#[derive(Debug, PartialEq)]
pub enum Token {
  Keyword(String),
  None,
}

pub fn token_at_position(
  document: &TextDocument,
  position: Position,
) -> Option<Token> {
  let line = document.get_line(position.line as usize);
  let mut start = position.character as usize;
  let mut end = position.character as usize;

  while start > 0 && is_token_char(line.chars().nth(start - 1).unwrap_or(' ')) {
    start -= 1;
  }

  while end < line.len() && is_token_char(line.chars().nth(end).unwrap_or(' ')) {
    end += 1;
  }

  if start == end {
    return Some(Token::None);
  }

  let token_text = line[start..end].to_string();
  Some(Token::Keyword(token_text))
}

pub fn match_keyword_from(token: Token) -> Vec<CompletionItem> {
  let mut completion_items = vec![];

  match token {
    Token::Keyword(partial_keyword) => {
      let keywords = vec![
        "link",
        "crate", "crate::", "crate://",
        "use",
        "pub",
        "dep"
      ];
      for keyword in keywords {
        if keyword.to_lowercase().contains(&partial_keyword.to_lowercase()) {
          match keyword {
            "link" => completion_items.extend(completion_items_for_link_keyword()),
            "crate" | "crate::" | "crate://" => {
              if "crate".contains(&partial_keyword.to_lowercase()) {
                completion_items.extend(completion_items_for_crate_keyword())
              }
            },
            "use" => completion_items.extend(completion_items_for_use_keyword()),
            "pub" | "dep" => completion_items.extend(completion_items_for_static_keywords()),
            _ => {}
          }
        }
      }

      if completion_items.is_empty() {
        return completion_items;
      }
    }
    Token::None => {
      return completion_items;
    }
  }

  completion_items.sort_by(|a, b| a.label.cmp(&b.label));
  completion_items.dedup_by(|a, b| a.label == b.label);

  completion_items
}


fn is_token_char(c: char) -> bool {
  c.is_alphanumeric() || c == '_' || c == ':' || c == '/' || c == '"'
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
    },
    CompletionItem {
      label: "use crate".to_string(),
      kind: Some(CompletionItemKind::KEYWORD),
      documentation: Some(Documentation::MarkupContent(
        MarkupContent {
          kind: MarkupKind::Markdown,
          value: ["# use crate",
            "The `use crate` keyword is used to import modules from the current crate.",
            "Example: ",
            "```rust",
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
