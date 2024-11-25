use crossbeam_channel::{unbounded, Sender};
use std::sync::Arc;

pub mod counter;

pub trait Source {
    fn get_input(&mut self) -> Option<Vec<u8>>;
}
