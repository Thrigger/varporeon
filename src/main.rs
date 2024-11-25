use log::info;

use std::thread;

fn main() {
    env_logger::init();
    info!("Starting Vaporeon in CLI mode");

    let config: &str = "
counter[5]>logOutput
counter[1]>logOutput";

    let mut thread_handles = vec![];
    for each in config.lines() {
        info!("Parsing line:{:?}", each);
        if each != "" {
            let mut new_chain = vaporeon::chain::NodeRoot::new_simple_cfg(each);
            thread_handles.push(
                thread::spawn(move || {new_chain.start()})
            );
        }
    }
    
    for handle in thread_handles {
        handle.join().unwrap();
    }
    // Will never be reached
}
