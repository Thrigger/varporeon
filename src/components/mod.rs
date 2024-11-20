use crossbeam_channel::Receiver;
use std::sync::Arc;
use log::{warn, error};

pub struct Logger {
}
pub struct LoggerOut {
}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }
}

impl LoggerOut {
    pub fn new() -> Self {
        LoggerOut {}
    }
}

impl Drain for Logger {
    fn run(&self, r: Arc<[u8]>) -> Option<Arc<[u8]>> {
        warn!("Got: {:?}", r);
        Some(r)
    }
}
impl Drain for LoggerOut {
    fn run(&self, r: Arc<[u8]>) -> Option<Arc<[u8]>> {
        warn!("Got: {:?}", r);
        None
    }
}

pub trait Drain {
    fn run(&self, rx: Arc<[u8]>) -> Option<Arc<[u8]>>;
}

