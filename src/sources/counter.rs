use super::Source;

use log::info;
use std::sync::Arc;
use std::thread;

pub struct Counter {
    counter: usize,
    stop: usize,
}

impl Counter {
    pub fn new(stop: usize) -> Self {
        Self { stop, counter: 0 }
    }
}

impl Source for Counter {
    fn get_input(&mut self) -> Option<Vec<u8>> {
        let i_u8 = self.counter.to_be_bytes().to_vec();
        info!("Counting: {:?}", i_u8);
        self.counter += 1;
        self.counter %= self.stop;
        
        Some(i_u8)
    }
}

