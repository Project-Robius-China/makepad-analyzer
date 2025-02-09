use std::sync::atomic::AtomicBool;

use lsp_types::{CompletionItem, Position, Url};
use makepad_analyzer_document::TextDocument;

use crate::utils::{self, Token};

pub struct Session {
  pub is_active: AtomicBool,
}

impl Session {
  pub fn new () -> Self {
    Session {
      is_active: AtomicBool::new(true),
    }
  }

  pub fn mark_inactived(&self) {
    self.is_active.store(false, std::sync::atomic::Ordering::Relaxed);
  }

  pub fn is_active(&self) -> bool {
    self.is_active.load(std::sync::atomic::Ordering::Relaxed)
  }

  pub fn completion_items(
    &self,
    document: &TextDocument,
    position: Position,
    trigger_char: &str,
  ) -> Option<Vec<CompletionItem>> {
    let _p = tracing::trace_span!("completion_items").entered();

    let shifted_position = Position {
      line: position.line,
      character: position.character - trigger_char.len() as u32 -1,
    };

    let token = utils::token_at_position(document, shifted_position);
    let completion_items = utils::match_keyword_from(token.unwrap_or(Token::None));
    Some(completion_items)
  }
}
