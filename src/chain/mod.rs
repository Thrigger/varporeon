use crate::components::{self, *};
use crate::sources::{self, *};

use std::thread;
use log::{debug, error, info, warn};
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

impl Node {
    fn create_component(cfg: &Vec<&str>) -> Option<Box<dyn Drain>> {
        if cfg[0] == "loggerOut" {
            return Some(Box::new(components::LoggerOut::new()));
        } else if cfg[0] == "logger" {
            return Some(Box::new(components::Logger::new()));
        }
        None
    }

    pub fn new(mut cfg: &toml::Value) -> Node {
        let Some(new_comp) = Self::create_component(&vec![&cfg["drain"].as_str().unwrap()]) else {
            error!("Config is wrong! ->{:#?}", cfg);
            panic!();
        };

        let mut node = Node {
            component: new_comp,
            next: vec![],
        };

        let nexts = cfg["next"].as_array().unwrap();
        for n in nexts {
            info!("this is next:{:#?}", n);
            info!("This is name:{:#?}", cfg[n.as_str().unwrap()]);

            node.next.push(Node::new(&cfg[n.as_str().unwrap()]));
        }

        node
    }

    pub fn new_simple_cfg(mut cfg: Vec<&str>) -> Node {
        let mut comp = cfg.remove(0).split("[").collect::<Vec<&str>>();
        if comp.len() > 1 {
            comp[1] = &comp[1][..comp[1].len()-1];
            debug!("Creating new component {} with cfg {}", comp[0], comp[1]);
        } else {
            debug!("Creating new component {} with no cfg", comp[0]);
        }

        let Some(new_comp) = Self::create_component(&comp) else {
            error!("Config is wrong! ->{:?}", comp);
            panic!();
        };

        if cfg.len() == 0 {
            // No child node so this is last.
            return Node { 
                    component: new_comp,
                    next: vec![],
                };
        }

        Node { 
            component: new_comp,
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
        } else {
            debug!("Could not produce any output from the input");
        }
    }
}

impl NodeRoot {
    pub fn new_chains(cfg: &toml::Table) -> Vec<NodeRoot> {
        let mut tmp_chains = vec![];
        for each in cfg {
            debug!("Parsing rootNode:{:#?}", each);

            tmp_chains.push(Self::new_chain(each));
        }
        tmp_chains
    }

    pub fn new_chain(cfg: (&String, &toml::Value)) -> NodeRoot {
        let mut nodeRoot = NodeRoot {
                component: Box::new(match cfg.1["source"].as_str().unwrap() {
                    "counter" => counter::Counter::new(Some(3)),
                    _ => {error!("input is invalid"); panic!("")},
                }),
                next: vec![],
            };
        let nexts = cfg.1["next"].as_array().unwrap();
        for n in nexts {
            info!("this is name of next:{:#?}", n);
            info!("This is content of next:{:#?}", cfg.1[n.as_str().unwrap()]);

            nodeRoot.next.push(Node::new(&cfg.1[n.as_str().unwrap()]));

            //let mut child_name = each.0.to_owned();
            //child_name.push_str(".");
            //child_name.push_str(n.as_str().unwrap());
            //info!("This is name:{:#?}", child_name);
            //info!("This is more:{:#?}", conf_file[&child_name]);
        }
        nodeRoot
    }

    pub fn new_simple_chains(cfg: &str) -> Vec<NodeRoot> {
            let mut tmp_chains = vec![];
            for each in cfg.lines() {
                debug!("Parsing line:{:?}", each);
                if each != "" && &each[..1] != "#" {
                    tmp_chains.push(Self::new_simple_chain(each));
                } else {
                    warn!("Faulty simple config:{}", cfg);
                }
            }
            tmp_chains
    }

    pub fn new_simple_chain(cfg: &str) -> NodeRoot {
        let mut parts = cfg.split(">").collect::<Vec<&str>>();
        if parts.len() == 0 {
            error!("cfg is invalide");
            panic!("cfg is invalide");
        } 

        let mut input = parts.remove(0).split("[").collect::<Vec<&str>>();
        let input_cfg;
        if input.len() > 1 {
            input[1] = &input[1][..input[1].len()-1];
            debug!("Creating new input {} with cfg {}", input[0], input[1]);
            input_cfg = Some(input[1].parse::<usize>().unwrap());
        } else {
            debug!("Creating new input {} with no cfg", input[0]);
            input_cfg = None;
        }

        NodeRoot {
            component: Box::new(match input[0] {
                "counter" => counter::Counter::new(input_cfg),
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
