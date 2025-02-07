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
