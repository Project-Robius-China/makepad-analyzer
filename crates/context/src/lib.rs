use std::sync::{Arc, Mutex, RwLock};

use makepad_analyzer_session::SessionManager;
use tower_lsp::Client;
use makepad_analyzer_config::Config;
use makepad_analyzer_plugin_manager::PluginManager;

pub struct AnalyzerContext {
  pub client: Option<Client>,
  pub config: Arc<RwLock<Config>>,

  pub session_manager: &'static SessionManager,
  pub plugin_manager: &'static PluginManager,
}
