mod capabilities;

pub use capabilities::*;

use lsp_types::{CompletionItem, Position, Url};
use makepad_analyzer_plugin_types::{MakepadAnalyzerPlugin, PluginCapability, PluginInfo};

struct LivePluginCapability;

impl PluginCapability for LivePluginCapability {
  fn handle_completion(
    &self,
    uri: &Url,
    position: Position,
    trigger_char: &str,
  ) -> Vec<CompletionItem> {
    capabilities::handle_completion(uri, position, trigger_char)
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
