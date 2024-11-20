use crate::components::{self, *};
use crate::sources::{self, *};

use std::thread;
use log::error;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use std::sync::Arc;

pub struct NodeRoot {
    /// source takes a trait object (a struct that implements the trait Source)
    source: Box<dyn sources::Source + Send + Sync>,

    next: Node,
}

pub struct Node {
    /// source takes a trait object (a struct that implements the trait Source)
    component: Box<dyn Drain>,

    /// drain takes a trait object (a struct that implements the trait Drain)
    next: Vec<Box<Node>>,
    //next: Option<Box<Node>>,
}

unsafe impl Send for Node{}
unsafe impl Sync for Node{}

impl Node {
    pub fn new() -> Node {
        Node { 
            component: Box::new(components::Logger::new()),
            next: vec![Box::new(Node::new_out())],
            //next: Some(Box::new(Node::new_out())),
        }
    }
    pub fn new_out() -> Node {
        Node { 
            component: Box::new(components::LoggerOut::new()),
            next: vec![],
            //next: None,
        }
    }
    pub fn init_node(&self, rx_from_parent: Receiver<Arc<[u8]>>) {
        if self.next.len() > 0 {
        //if self.next.is_some() {
            // This is not end node
            let (tx, rx_to_next) = crossbeam_channel::unbounded();
            //let _drain_thread_join_handle = thread::spawn(move || {
            //    self.next.as_ref().unwrap().init_node(rx_to_next);
            //});
            for child in &self.next {
                let _drain_thread_join_handle = thread::spawn(move || {
                    child.init_node(rx_to_next);
                });
            }

            loop {
                let Ok(r) = rx_from_parent.recv() else {
                    error!("Sender closed channel"); 
                    break;
                };
                if let Some(buf) = self.component.run(r) {
                    tx.send(buf);
                }
            }
        
        } else {
            // This is an output
            loop {
                let Ok(r) = rx_from_parent.recv() else {
                    error!("Sender closed channel"); 
                    break;
                };
                self.component.run(r);
            }
        }
    }
}

impl NodeRoot {
    pub fn new() -> NodeRoot {
        NodeRoot { 
            source: Box::new(counter::Counter::new(5)),
            next: Node::new(),
        }
    }

    pub fn start(self) {
        let (tx, rx) = crossbeam_channel::unbounded();
        let _source_thread_join_handle = thread::spawn(move || {
            self.source.start(tx)
        });
        let _drain_thread_join_handle = thread::spawn(move || {
            self.next.init_node(rx);
        });

        // TODO add better monitoring of threads. It would be nice to replace .join() since it is
        // blocking.
        let res = _drain_thread_join_handle.join();
        println!("Res: {:?}", res);
    }
}
