use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use once_cell::sync::Lazy;
use serde_yaml::from_reader;

use config::Config;

mod config;
mod git;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let file = File::open("config.yaml").expect("Failed to open config file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Failed to read config file");
    from_reader(contents.as_bytes()).expect("Failed to deserialize config file")
});

fn main() -> Result<(), io::Error> {
    for repo in &CONFIG.clusters {
        git::init(&repo.path);
    }

    Ok(())
}
