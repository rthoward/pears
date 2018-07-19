extern crate clap;

#[macro_use]
extern crate serde_derive;

mod config;
mod types;
mod github;
mod term;

use clap::{App, Arg};
use config::read_config_file;
use github::fetch_prs;
use term::{display_pr, display_repo};


fn main() {
    let matches = App::new("pears")
        .version("0.1")
        .author("Richard Howard <richard@howard.io>")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Set a custom config file")
                .takes_value(true)
                .default_value("~/.config/pears/pears.json"),
        )
        .get_matches();
    let config = read_config_file(matches.value_of("config").unwrap())
        .expect("Could not parse config file.");

    for repo in config.repos {
        display_repo(&repo);
        let prs = fetch_prs(&repo).expect("Could not reach GitHub API.");

        for pr in prs {
            display_pr(&pr);
        }
    }
}
