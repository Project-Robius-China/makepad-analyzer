use std::{collections::VecDeque, path::PathBuf, sync::Arc};

use dashmap::{mapref::{entry, multiple::RefMulti}, DashMap};
use parking_lot::Mutex;

use crate::Session;

pub struct LRUSessionCache {
  pub sessions: DashMap<PathBuf, Arc<Session>>,
  usage_order: Mutex<VecDeque<PathBuf>>,
  capacity: usize,
}

impl LRUSessionCache {
  pub fn new(capacity: usize) -> Self {
    LRUSessionCache {
      sessions: DashMap::new(),
      usage_order: Mutex::new(VecDeque::with_capacity(capacity)),
      capacity,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = RefMulti<'_, PathBuf, Arc<Session>>> {
    self.sessions.iter()
  }

  pub fn get(&self, path: &PathBuf) -> Option<Arc<Session>> {
    if let Some(session) = self.sessions.try_get(path).try_unwrap() {
      if self.sessions.len() >= self.capacity {
        self.move_to_front(path);
      }
      Some(session.clone())
    } else {
      None
    }
  }

  pub fn insert(&self, path: PathBuf, session: Arc<Session>) {
    if let Some(mut entry) = self.sessions.get_mut(&path) {
      // Session already exists, update it
      *entry = session;
      self.move_to_front(&path);
    } else {
      if self.sessions.len() >= self.capacity {
          self.evict_least_recently_used();
      }
      self.sessions.insert(path.clone(), session);
      let mut order = self.usage_order.lock();
      order.push_front(path);
    }
  }

  pub async fn cleanup_sessions(&self) {
    let inactive_sessions = self.collect_inactive_sessions();

    if inactive_sessions.is_empty() {
      return;
    }

    // Remove inactive sessions
    for path in &inactive_sessions {
      self.sessions.remove(path);
      tracing::info!("Removed inactive session: {:?}", path);
    }

    // Remove inactive sessions from usage order
    let mut usage_order = self.usage_order.lock();
    usage_order.retain(|path| !inactive_sessions.contains(path));
  }

  pub fn mark_session_inactive(&self, path: &PathBuf) {
    for entry in self.sessions.iter() {
      if entry.key() == path {
        entry.value().inactive();
        break;
      }
    }
  }

  pub fn current_usage(&self) -> f32 {
    self.sessions.len() as f32 / self.capacity as f32
  }

  pub fn capacity(&self) -> usize {
    self.capacity
  }

  fn collect_inactive_sessions(&self) -> Vec<PathBuf> {
    self.sessions
      .iter()
      .filter_map(|entry| {
        if !entry.value().is_active.load(std::sync::atomic::Ordering::Relaxed) {
          Some(entry.key().clone())
        } else {
          None
        }
      })
      .collect()
  }

  fn move_to_front(&self, path: &PathBuf) {
    tracing::trace!("Moving path to front of usage order: {:?}", path);
    let mut order = self.usage_order.lock();
    if let Some(index) = order.iter().position(|p| p == path) {
      order.remove(index);
    }
    order.push_front(path.clone());
  }

  fn evict_least_recently_used(&self) {
    let mut order = self.usage_order.lock();
    if let Some(old_path) = order.pop_back() {
      tracing::trace!(
          "Cache at capacity. Evicting least used session: {:?}",
          old_path
      );
      self.sessions.remove(&old_path);
    }
  }
}
