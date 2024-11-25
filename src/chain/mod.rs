use crate::components::{self, *};
use crate::sources::{self, *};

use std::thread;
use log::{debug, error};
use bus::{Bus, BusReader};
use std::sync::Arc;
use std::time;

pub struct NodeRoot {
    /// source takes a trait object (a struct that implements the trait Source)
    component: Box<dyn sources::Source + Send + Sync>,

    next: Vec<Node>,
}

pub struct Node {
    /// source takes a trait object (a struct that implements the trait Source)
    component: Box<dyn Drain>,

    /// drain takes a trait object (a struct that implements the trait Drain)
    next: Vec<Node>,
}

unsafe impl Send for Node{}
//unsafe impl Sync for Node{}

impl Node {
    pub fn new() -> Node {
        Node { 
            component: Box::new(components::Logger::new()),
            next: vec![],
        }
    }

    pub fn new_simple_cfg(mut cfg: Vec<&str>) -> Node {
        let mut comp = cfg.remove(0).split("[").collect::<Vec<&str>>();
        if comp.len() > 1 {
            comp[1] = &comp[1][..comp[1].len()-1];
            debug!("Creating new component {} with cfg {}", comp[0], comp[1]);
        } else {
            debug!("Creating new component {} with no cfg", comp[0]);
        }

        let new_comp = match comp[0] {
        //    "logger" => components::Logger::new(),
            "loggerOut" => components::LoggerOut::new(),
            _ => panic!("not yet implemented"),
        };

        if cfg.len() == 0 {
            return Node { 
                    component: Box::new(new_comp),
                    next: vec![],
                };
        }

        Node { 
            component: Box::new(new_comp),
            next: vec![Node::new_simple_cfg(cfg)],
        }
    }

    pub fn new_out() -> Node {
        Node { 
            component: Box::new(components::LoggerOut::new()),
            next: vec![],
        }
    }

    pub fn run(&self, data: &[u8]) {
        if let Some(new_data) = self.component.run(data) {
            for each in &self.next {
                each.run(data);
            }
        }
    }
}

impl NodeRoot {
    pub fn new() -> NodeRoot {
        NodeRoot {
            component: Box::new(counter::Counter::new(5)),
            next: vec![Node::new_out()],
        }
    }

    pub fn new_simple_cfg(cfg: &str) -> NodeRoot {
        let mut parts = cfg.split(">").collect::<Vec<&str>>();
        if parts.len() == 0 {
            error!("cfg is invalide");
            panic!("cfg is invalide");
        } 

        let mut input = parts.remove(0).split("[").collect::<Vec<&str>>();
        input[1] = &input[1][..input[1].len()-1];
        debug!("Creating new input {} with cfg {}", input[0], input[1]);

        NodeRoot {
            component: Box::new(match input[0] {
                "counter" => counter::Counter::new(5),
                _ => {error!("input is invalid"); panic!("")},
            }),
            next: vec![Node::new_simple_cfg(parts)],
        }
    }

    pub fn start(&mut self) {
        loop {
            if let Some(buf) = self.component.get_input() {
                for each in &self.next {
                    each.run(&buf);
                }
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
