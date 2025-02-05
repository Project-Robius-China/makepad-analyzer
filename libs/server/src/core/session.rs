use std::path::PathBuf;
use tower_lsp::lsp_types::{CompletionItem, Position, Url};

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

    let _p = tracing::trace_span!("completion_items").entered();

    let _shifted_position = Position {
      line: position.line,
      character: position.character - trigger_char.len() as u32 - 1,
    };

    // TODO: request completions from plugins
    // for plugin in self.applied_plugins.iter() {
    //   plugin.completion_items(uri, position, trigger_char);
    // }

    let completion_items =
      makepad_analyzer_plugin_live::completion_items(uri, position, trigger_char);

    completion_items
  }
}
