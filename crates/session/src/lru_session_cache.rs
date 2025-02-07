use std::{collections::VecDeque, path::PathBuf, sync::{Arc, Mutex}};

use dashmap::DashMap;

use crate::Session;

pub struct LRUSessionCache {
  pub sessions: DashMap<PathBuf, Arc<Session>>,
  usage_order: Mutex<VecDeque<PathBuf>>,
  capacity: usize,
}

impl LRUSessionCache {
  pub fn new(capacity: usize) -> Self {
    Self {
      sessions: DashMap::new(),
      usage_order: Mutex::new(VecDeque::with_capacity(capacity)),
      capacity,
    }
  }

  pub fn get(&self, path: &PathBuf) -> Option<Arc<Session>> {
    if let Some(session) = self.sessions.get(path) {
      self.move_to_front(path);
      Some(session.value().clone())
    } else {
      None
    }
  }

  pub fn insert(&self, path: PathBuf, session: Arc<Session>) {
    let mut order = self.usage_order.lock().unwrap();

    if self.sessions.contains_key(&path) {
      self.sessions.insert(path.clone(), session);
      order.retain(|p| p != &path);
    } else {
      if self.sessions.len() >= self.capacity {
        self.evict_least_recently_used(&mut order);
      }
      self.sessions.insert(path.clone(), session);
    }
    order.push_front(path);
  }

  pub fn cleanup_sessions(&self) {
    let mut order = self.usage_order.lock().unwrap();
    order.retain(|path| {
      if let Some(session) = self.sessions.get(path) {
        let session = session.value();
        session.is_vaild()
      } else {
        false
      }
    });
  }

  fn move_to_front(&self, path: &PathBuf) {
    let mut order = self.usage_order.lock().unwrap();
    order.retain(|p| p != path);
    order.push_front(path.clone());
  }

  fn evict_least_recently_used(&self, order: &mut VecDeque<PathBuf>) {
    if let Some(old_path) = order.pop_back() {
      self.sessions.remove(&old_path);
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_lru_session_cache_capacity() {
    let cache = LRUSessionCache::new(2);
    let path1 = PathBuf::from("/path/session1");
    let path2 = PathBuf::from("/path/session2");
    let path3 = PathBuf::from("/path/session3");

    let session1 = Arc::new(Session::new());
    let session2 = Arc::new(Session::new());
    let session3 = Arc::new(Session::new());

    cache.insert(path1.clone(), session1.clone());
    cache.insert(path2.clone(), session2.clone());
    cache.insert(path3.clone(), session3.clone());

    assert!(cache.get(&path1).is_none());
    assert!(Arc::ptr_eq(&cache.get(&path2).unwrap(), &session2));
    assert!(Arc::ptr_eq(&cache.get(&path3).unwrap(), &session3));
  }

  #[test]
  fn test_lru_session_cache_insertion_and_find() {
    let cache = LRUSessionCache::new(2);
    let path1 = PathBuf::from("/path/session1");
    let path2 = PathBuf::from("/path/session2");

    let session1 = Arc::new(Session::new());
    let session2 = Arc::new(Session::new());

    cache.insert(path1.clone(), session1.clone());
    cache.insert(path2.clone(), session2.clone());

    assert!(Arc::ptr_eq(&cache.get(&path1).unwrap(), &session1));
    assert!(Arc::ptr_eq(&cache.get(&path2).unwrap(), &session2));
  }

  #[test]
  fn test_lru_session_cache_update_order() {
    let cache = LRUSessionCache::new(2);
    let path1 = PathBuf::from("/path/session1");
    let path2 = PathBuf::from("/path/session2");
    let path3 = PathBuf::from("/path/session3");

    let session1 = Arc::new(Session::new());
    let session2 = Arc::new(Session::new());
    let session3 = Arc::new(Session::new());

    cache.insert(path1.clone(), session1.clone());
    cache.insert(path2.clone(), session2.clone());

    cache.get(&path1);

    cache.insert(path3.clone(), session3.clone());

    assert!(cache.get(&path1).is_some());
    assert!(cache.get(&path2).is_none());
    assert!(cache.get(&path3).is_some());
  }
}
