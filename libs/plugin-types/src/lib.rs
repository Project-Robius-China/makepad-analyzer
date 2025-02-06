use lsp_types::{CompletionItem, Position, Url};

#[derive(Debug, Clone)]
pub struct PluginInfo {
  pub name: String,
  pub description: String,
  pub version: String,
}

pub trait PluginCapability: Send + Sync {
  fn handle_completion(
    &self,
    uri: &Url,
    position: Position,
    trigger_char: &str,
  ) -> Vec<CompletionItem>;
}

pub trait MakepadAnalyzerPlugin: Send + Sync {
  fn plugin_info(&self) -> PluginInfo;
  fn capabilities(&self) -> &dyn PluginCapability;
}
