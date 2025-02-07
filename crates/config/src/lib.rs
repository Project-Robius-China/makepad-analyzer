mod client;
mod logging;

pub use client::*;
pub use logging::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
  #[serde(default)]
  pub client: LSPClient,
  #[serde(default)]
  pub logging: LoggingConfig,
}
