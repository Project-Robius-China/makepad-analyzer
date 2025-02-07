use std::sync::Arc;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use makepad_analyzer_plugin_live::MakepadAnalyzerLivePlugin;
use makepad_analyzer_plugin_types::{MakepadAnalyzerPlugin, PluginInfo, CompletionItem, Position, Url};

pub static PLUGIN_MANAGER: Lazy<PluginManager> = Lazy::new(||
  PluginManagerBuilder::new()
    .apply_plugin(Arc::new(MakepadAnalyzerLivePlugin::new()))
    .finish()
);

pub struct PluginManagerBuilder {
  plugin_manager: PluginManager,
}

impl PluginManagerBuilder {
  pub fn new() -> Self {
    Self {
        plugin_manager: PluginManager {
          plugins: Arc::new(Mutex::new(Vec::new())),
        },
    }
  }

  pub fn apply_plugin(self, plugin: Arc<dyn MakepadAnalyzerPlugin>) -> Self {
    self.plugin_manager.register_plugin(plugin);
    self
  }

  pub fn finish(self) -> PluginManager {
    self.plugin_manager
  }
}

pub struct PluginManager {
  plugins: Arc<Mutex<Vec<Arc<dyn MakepadAnalyzerPlugin>>>>,
}

impl PluginManager {
  pub fn get_all_registered_plugin_info(&self) -> Vec<PluginInfo> {
    let plugins = self.get_plugins();
    plugins.iter().map(|plugin| plugin.plugin_info()).collect()
  }

  pub fn get_registered_plugin_info(&self, plugin_name: &str) -> Option<PluginInfo> {
    let plugins = self.get_plugins();
    for plugin in plugins {
      let info = plugin.plugin_info();
      if info.name == plugin_name {
        return Some(info);
      }
    }
    None
  }

  pub fn get_plugins(&self) -> Vec<Arc<dyn MakepadAnalyzerPlugin>> {
    self.plugins.lock().clone()
  }

  fn register_plugin(&self, plugin: Arc<dyn MakepadAnalyzerPlugin>) {
    self.plugins.lock().push(plugin);
  }

  pub fn handle_completion(
    &self,
    uri: &Url,
    position: Position,
    trigger_char: &str,
  ) -> Vec<CompletionItem> {
    let plugins =  self.get_plugins();
    let mut completion_items = Vec::new();

    for plugin in plugins {
      let capability = plugin.capabilities();
      let items = capability.handle_completion(uri, position, trigger_char);
      completion_items.extend(items);
    }
    completion_items
  }
}
