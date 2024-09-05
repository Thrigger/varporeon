use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time;
use std::thread;

pub struct Counter {
    stop: usize,
}

impl Counter {
    pub fn new(stop: usize) -> Self {
        Self { stop }
    }
}

impl Source for Counter {
    fn start(&self, sender: Sender<Arc<[u8]>>) {
        let mut i: usize = 0;
        loop {
            let i_u8 = i.to_be_bytes();
            sender.send(i_u8.into()).unwrap();
            i += 1;
            i %= self.stop;
            
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

pub trait Source {
    fn start(&self, sender: Sender<Arc<[u8]>>);
}

