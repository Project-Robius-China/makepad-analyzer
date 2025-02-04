use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
  #[serde(default)]
  pub client: LSPClient,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LSPClient {
  VSCode,
  MakepadStudio,
  #[serde(other)]
  #[default]
  Other,
}
