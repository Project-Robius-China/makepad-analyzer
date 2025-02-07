mod logging;

use logging::LoggingConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LSPClient {
  VSCode,
  MakepadStudio,
  #[serde(other)]
  #[default]
  Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
  #[serde(default)]
  pub client: LSPClient,
  #[serde(default)]
  pub logging: LoggingConfig,
}
