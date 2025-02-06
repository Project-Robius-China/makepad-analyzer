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
    let _p = tracing::trace_span!("completion_items").entered();

    let _shifted_position = Position {
      line: position.line,
      character: position.character - trigger_char.len() as u32 - 1,
    };

    // Apply plugins and get completion items, but now we only have one plugin.
    // TODO: if we have multiple plugins, we should apply them all, and then merge the completion items.

    // Maybe like this: PluginManager::get_plugins().capabilities().handle_completion(uri, position, trigger_char)
    // Then we just return the merged completion items.
    // The merge operation should be done by the PluginManager.
    // The merged completion items should be sorted by the order of the plugins.
    let completion_items =
      makepad_analyzer_plugin_live::handle_completion(uri, position, trigger_char);

    Some(completion_items)
  }
}
