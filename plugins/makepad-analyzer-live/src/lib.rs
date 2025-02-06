mod live;

pub use live::*;
use lsp_types::{CompletionItem, Position, Url};
use makepad_analyzer_plugin_manager::{MakepadAnalyzerPlugin, PluginCapability, PluginInfo};

struct LivePluginCapability;

impl PluginCapability for LivePluginCapability {
  fn handle_completion(
    &self,
    uri: &Url,
    position: Position,
    trigger_char: &str,
  ) -> Vec<CompletionItem> {
    // Here we delegate the handling of completions to the live or shader module.
    // Then merge the results from the live and shader modules, but now we first consider the live module.
    live::handle_completion(uri, position, trigger_char)
  }
}

pub struct MakepadAnalyzerLivePlugin {
  plugin_info: PluginInfo,
  capability: LivePluginCapability,
}

impl MakepadAnalyzerPlugin for MakepadAnalyzerLivePlugin {
  fn plugin_info(&self) -> PluginInfo {
    self.plugin_info.clone()
  }

  fn capabilities(&self) -> &dyn PluginCapability {
    &self.capability
  }
}

impl MakepadAnalyzerLivePlugin {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for MakepadAnalyzerLivePlugin {
  fn default() -> Self {
    MakepadAnalyzerLivePlugin {
      plugin_info: PluginInfo {
        name: "Makepad Analyzer Live Plugin".to_string(),
        description: "A plugin for the Makepad Analyzer that provides live completion".to_string(),
        version: "0.0.1".to_string(),
      },
      capability: LivePluginCapability,
    }
  }
}
