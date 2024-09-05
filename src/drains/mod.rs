use std::sync::mpsc::Receiver;
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
    fn start(&self, rx: Receiver<Arc<[u8]>>) {
        loop {
            match rx.recv() {
                Err(e) => {error!("Sender closed channel: {e}"); break;},
                Ok(r) => warn!("Got: {:?}", r),
            };
        }
    }
}

pub trait Drain {
    fn start(&self, rx: Receiver<Arc<[u8]>>);
}

