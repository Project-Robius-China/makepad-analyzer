use makepad_analyzer_plugin_manager::PluginManager;
use tower_lsp::lsp_types::{CompletionItem, Position, Url};

pub struct Session {
  plugin_manager: &'static PluginManager,
}

impl Session {
  pub fn new(
    plugin_manager: &'static PluginManager,
  ) -> Self {
    Self {
      plugin_manager,
    }
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
    let mut completion_items = Vec::new();

    for plugin in self.plugin_manager.get_plugins() {
      // TODO: we need to judge whether the plugin is applied to the current session.
      // Otherwise, we also should to check the plugin is enabled or not.
      let plugin_completion_items = plugin.capabilities().handle_completion(uri, position, trigger_char);
      completion_items.extend(plugin_completion_items);
    }

    Some(completion_items)
  }
}
