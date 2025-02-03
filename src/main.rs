use std::fs;
use std::path::PathBuf;
use std::thread;

use clap::Parser;
use log::{debug, info};
use serde_derive::Deserialize;

use vaporeon::chain;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    chains_config: PathBuf,

    #[arg(short, long)]
    use_simple_cfg: bool,
}

/// Struct for the Toml file to parse
#[derive(Deserialize,Debug)]
struct ConfigFile {
}

fn main() {
    env_logger::init();
    info!("Starting Vaporeon in CLI mode");

    let cli = Cli::parse();
    let contents = fs::read_to_string(cli.chains_config).unwrap();

    let chains: Vec<chain::NodeRoot> = match cli.use_simple_cfg {
        true => vaporeon::chain::NodeRoot::new_simple_chains(&contents),
        false => {
            let conf_file: ConfigFile = toml::from_str(&contents).expect("Parse toml file");
            info!("toml-file as config_file:\n{:#?}", conf_file);
            vec![]
        },
    };

    let mut thread_handles = vec![];
    for mut each in chains {
        thread_handles.push(
            thread::spawn(move || {each.start()})
        );
    }
    
    for handle in thread_handles {
        handle.join().unwrap();
    }
    // Will never be reached
}
