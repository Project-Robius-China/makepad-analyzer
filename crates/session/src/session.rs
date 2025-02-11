use std::{ffi::OsStr, fs, path::{self, Path, PathBuf}, sync::atomic::{AtomicBool, Ordering::Relaxed}};

use lsp_types::{CompletionItem, CompletionItemKind, Position};
use makepad_analyzer_core::errors::MakepadAnalyzerError;
use makepad_analyzer_document::{Documents, TextDocument};
use tracing::info;
use url::Url;

use crate::SyncWorkspace;

pub type ProjectDirectory = PathBuf;

#[derive(Debug)]
pub struct Session {
  pub is_active: AtomicBool,
  pub sync: SyncWorkspace,
}

impl Session {
  pub fn new () -> Self {
    Session {
      sync: SyncWorkspace::new(),
      is_active: AtomicBool::new(true),
    }
  }

  pub async fn init(
    &self,
    uri: &Url,
    documents: &Documents,
  ) -> Result<ProjectDirectory, MakepadAnalyzerError> {
    let manifest_dir = PathBuf::from(uri.path());

    self.sync.create_temp_dir_from_workspace(&manifest_dir)?;
    self.sync.clone_manifest_dir_to_temp()?;
    let _ = self.store_project_files(documents).await?;
    // self.sync.watch_and_sync_manifest();
    self.sync.manifest_dir().map_err(Into::into)
  }

  pub fn mark_inactived(&self) {
    self.is_active.store(false, Relaxed);
  }

  pub fn is_active(&self) -> bool {
    self.is_active.load(Relaxed)
  }

  async fn store_project_files(
    &self,
    documents: &Documents,
  ) -> Result<(), MakepadAnalyzerError> {
    let temp_dir = self.sync.temp_dir()?;
    for path in get_project_files(temp_dir).iter().filter_map(|fp| fp.to_str()) {
      documents.store_document(TextDocument::build_from_path(path).await?)?;
    }

    Ok(())
  }

  pub fn completion_items(
    &self,
    uri: &Url,
    position: Position,
    trigger_char: &str,
  ) -> Option<Vec<CompletionItem>> {
    let shifted_position = Position {
      line: position.line,
      character: position.character - trigger_char.len() as u32 - 1,
    };
    tracing::info!("uri: {:?}", uri);
    let completion_items = vec![
      CompletionItem {
        label: "Hello".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        detail: Some("Hello".to_string()),
        ..CompletionItem::default()
      },
      CompletionItem {
        label: "World".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        detail: Some("World".to_string()),
        ..CompletionItem::default()
      },
    ];

    Some(completion_items)
  }
}

fn get_project_files(path: PathBuf) -> Vec<PathBuf> {
  let mut files = vec![];
  let mut dir_entries = vec![path];

  while let Some(next_dir) = dir_entries.pop() {
    if let Ok(read_dir) = fs::read_dir(&next_dir) {
      for entry in read_dir.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
          dir_entries.push(path);
        } else if is_rust_file(&path) {
          files.push(path);
        }
      }
    }
  }
  files
}

pub fn is_rust_file(file: &Path) -> bool {
  file.is_file() && file.extension() == Some(OsStr::new("rs"))
}
