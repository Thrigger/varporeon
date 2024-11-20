use crossbeam_channel::Receiver;
use std::sync::Arc;
use log::{warn, error};

pub struct Logger {
}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }
}

impl Drain for Logger {
    fn run(&self, r: Arc<[u8]>) {
        warn!("Got: {:?}", r);
    }
}

pub trait Drain {
    fn run(&self, rx: Arc<[u8]>);
}

