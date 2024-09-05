use crate::drains;
use crate::sources;

use std::thread;
use std::sync::mpsc;

pub struct Chain {
    /// source takes a trait object (a struct that implements the trait Source)
    source: Box<dyn sources::Source + Send + Sync>,

    /// drain takes a trait object (a struct that implements the trait Drain)
    drain: Box<dyn drains::Drain + Send + Sync>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain { 
            source: Box::new(sources::Counter::new(5)),
            drain: Box::new(drains::Logger::new()),
        }
    }

    pub fn start(self) {
        let (tx, rx) = mpsc::channel();
        let _source_thread_join_handle = thread::spawn(move || {
            self.source.start(tx)
        });
        let _drain_thread_join_handle = thread::spawn(move || {
            self.drain.start(rx);
        });

        // TODO add better monitoring of threads. It would be nice to replace .join() since it is
        // blocking.
        let res = _drain_thread_join_handle.join();
        println!("Res: {:?}", res);
    }
}
