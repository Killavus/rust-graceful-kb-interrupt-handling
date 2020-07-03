pub struct Discord {
    finish: AtomicBool,
}
use super::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
};

impl Discord {
    pub fn new(_tx: Sender<Command>) -> Arc<Self> {
        Arc::new(Self {
            finish: AtomicBool::new(false),
        })
    }

    pub fn start(&self) {
        while !self.finish.load(Ordering::Relaxed) {}
    }

    pub fn shutdown(&self) {
        self.finish.store(true, Ordering::Relaxed);
    }
}
