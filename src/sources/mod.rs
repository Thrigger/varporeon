use crossbeam_channel::{unbounded, Sender};
use std::sync::Arc;

pub mod counter;

pub trait Source {
    fn start(&self, sender: Sender<Arc<[u8]>>);
}
