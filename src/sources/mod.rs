pub mod counter;
//pub mod tcp_client;

use std::sync::mpsc::Sender;
use std::sync::Arc;

pub trait Source {
    fn start(&self, sender: Sender<Arc<[u8]>>);
}
