use std::path::PathBuf;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation, MarkupContent, Position, Url};

use crate::error::MakepadAnalyzerServrError;

#[derive(Debug)]
pub struct Session {
  /// A list of plugins to be used in the current session.
  pub applied_plugins: Vec<String>,
}

impl Default for Session {
  fn default() -> Self {
    Self::new()
  }
}

impl Session {
  pub fn new() -> Self {
    Self {
      applied_plugins: Vec::new(),
    }
  }

  pub fn init(
    &mut self,
  ) -> Result<PathBuf, MakepadAnalyzerServrError> {
    todo!()
  }

  pub fn completion_items(
    &self,
    uri: &Url,
    position: Position,
    trigger_char: &str,
  ) -> Option<Vec<CompletionItem>> {
    tracing::info!(
      "Completion request for: {:#?}, position: {:#?}, trigger_char: {:#?}",
      uri,
      position,
      trigger_char
    );

    let completion_items = vec![
      CompletionItem {
        label: "Some completion item".to_string(),
        kind: Some(CompletionItemKind::VARIABLE),
        detail: Some("Some detail".to_string()),
        documentation: Some(Documentation::MarkupContent(
          MarkupContent {
            kind: tower_lsp::lsp_types::MarkupKind::Markdown,
            value: "Some documentation".to_string(),
          },
        )),
        ..Default::default()
      },
      CompletionItem {
        label: "Another completion item".to_string(),
        kind: Some(CompletionItemKind::VARIABLE),
        detail: Some("Another detail".to_string()),
        documentation: Some(Documentation::MarkupContent(
          MarkupContent {
            kind: tower_lsp::lsp_types::MarkupKind::Markdown,
            value: "Another documentation".to_string(),
          },
        )),
        ..Default::default()
      },
    ];

    Some(completion_items)
  }
}
