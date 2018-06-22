extern crate clap;
extern crate serde_json;
extern crate shellexpand;

use clap::{Arg, App};
use std::fs::{File, canonicalize};
use std::path::PathBuf;
use std::io::prelude::*;

fn read_config_file(path: &str) -> String {
    let expanded_path = PathBuf::from(shellexpand::tilde(path).to_string());
    let mut f = File::open(expanded_path)
        .expect("Could not open config file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Could not read config file.");
    contents
}

fn main() {
    let matches = App::new("pears")
                          .version("0.1")
                          .author("Richard Howard <richard@howard.io>")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Set a custom config file")
                               .takes_value(true)
                               .default_value("~/.config/pears/pears.json"))
                          .get_matches();
    let config = read_config_file(matches.value_of("config").unwrap());
    println!("{}", config);
}
