// use std::path::PathBuf;

// use dashmap::DashMap;
// use lsp_types::Range;

// #[derive(Debug, Default)]
// pub struct TokenMap(DashMap<TokenIdent, Token>);

// impl std::ops::Deref for TokenMap {
//   type Target = DashMap<TokenIdent, Token>;

//   fn deref(&self) -> &Self::Target {
//     &self.0
//   }
// }


// #[derive(Debug, Default, Clone, PartialEq, Eq)]
// pub struct TokenIdent {
//   pub name: String,
//   pub range: Range,
//   pub path: Option<PathBuf>,
// }


// #[derive(Debug, Clone)]
// pub struct Token {
//   pub ast_node: TokenAstNode,
// }
