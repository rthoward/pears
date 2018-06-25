extern crate serde_json;

extern crate shellexpand;

use self::serde_json::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use types::Config;

pub fn read_config_file(path: &str) -> Result<Config, Error> {
    let expanded_path = PathBuf::from(shellexpand::tilde(path).to_string());
    let mut f = File::open(expanded_path).expect("Could not open config file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Could not read config file.");

    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}
