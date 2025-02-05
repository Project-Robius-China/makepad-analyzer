use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
  #[serde(default)]
  pub client: LSPClient,
  #[serde(default)]
  pub logging: LoggingConfig,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LSPClient {
  VSCode,
  MakepadStudio,
  #[serde(other)]
  #[default]
  Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggingConfig {
  #[serde(with = "LevelFilterDef")]
  pub level: LevelFilter,
}

impl Default for LoggingConfig {
  fn default() -> Self {
      Self {
        level: LevelFilter::OFF
      }
  }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "LevelFilter")]
#[allow(clippy::upper_case_acronyms)]
enum LevelFilterDef {
  OFF,
  ERROR,
  WARN,
  INFO,
  DEBUG,
  TRACE,
}
