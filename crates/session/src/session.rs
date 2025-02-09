use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[derive(Debug)]
pub struct Session {
  pub is_active: AtomicBool,
}

impl Session {
  pub fn new () -> Self {
    Session {
      is_active: AtomicBool::new(true),
    }
  }

  pub fn mark_inactived(&self) {
    self.is_active.store(false, Relaxed);
  }

  pub fn is_active(&self) -> bool {
    self.is_active.load(Relaxed)
  }
}
