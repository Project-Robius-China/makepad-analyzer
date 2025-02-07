use std::sync::{Arc, RwLock};

use tower_lsp::Client;
use makepad_analyzer_config::Config;
use makepad_analyzer_plugin_manager::PluginManager;
use makepad_analyzer_session::SessionManager;

pub struct AnalyzerContext {
  pub client: Option<Client>,
  pub config: Arc<RwLock<Config>>,

  /// The session manager for the Makepad Analyzer
  pub session_manager: &'static SessionManager,
  /// The plugin manager for the Makepad Analyzer
  pub plugin_manager: &'static PluginManager,
}

// impl Default for AnalyzerContext {
//   fn default() -> Self {
//       let state = AnalyzerContext {
//         client: None,
//         config: Arc::new(RwLock::new(Config::default())),
//         session_manager: SessionManager::default(),
//         plugin_manager: PluginManager::default(),
//       }
//   }
// }
