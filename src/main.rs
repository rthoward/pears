extern crate clap;
extern crate console;

#[macro_use]
extern crate serde_derive;

mod config;
mod display;
mod github;
mod types;

use clap::{App, Arg};
use config::read_config_file;
use display::PearsDisplay;
use github::{GithubAPI, GithubRESTAPI};

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

    let display = PearsDisplay::new();

    for repo in config.repos {
        let api = GithubRESTAPI {};
        let prs = api.fetch_prs(&repo).expect("Could not reach GitHub API.");

        if !prs.is_empty() {
            display.repo(&repo);
        }

        for pr in prs {
            display.pr(&pr);
        }
    }
}
