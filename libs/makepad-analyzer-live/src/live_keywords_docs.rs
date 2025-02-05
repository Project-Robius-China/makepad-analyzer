use std::{collections::HashMap, vec};

#[derive(Debug, Default)]
pub struct LiveKeywordsDocs(HashMap<String, String>);

impl LiveKeywordsDocs {
  pub fn new() -> Self {
    let mut keyword_docs = HashMap::new();

    let keyworkds = vec![
      "pub".to_string(),
      "link::".to_string(),
      "crate::".to_string(),
      "use".to_string(),
      "dep".to_string(),
    ];

    for keyword in keyworkds {
      // TODO: To find the actual documentation for these keywords
      keyword_docs.insert(keyword, "Some detail".to_string());
    }

    Self(keyword_docs)
  }
}

impl std::ops::Deref for LiveKeywordsDocs {
  type Target = HashMap<String, String>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
