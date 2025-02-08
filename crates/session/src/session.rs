use std::sync::atomic::AtomicBool;

pub struct Session {
  pub is_active: AtomicBool,
}

impl Session {
  pub fn new () -> Self {
    Session {
      is_active: AtomicBool::new(true),
    }
  }

  pub fn inactive(&self) {
    self.is_active.store(false, std::sync::atomic::Ordering::Relaxed);
  }
}
