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
    fn run(&self, r: &[u8]) -> Option<Vec<u8>> {
        warn!("Logger - Got: {:?}", r);
        Some(r.to_vec())
    }
}
impl Drain for LoggerOut {
    fn run(&self, r: &[u8]) -> Option<Vec<u8>> {
        warn!("LoggerOut - Got: {:?}", r);
        None
    }
}

pub trait Drain {
    fn run(&self, rx: &[u8]) -> Option<Vec<u8>>;
}

