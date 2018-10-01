extern crate chrono;
extern crate clap;
extern crate console;
extern crate git2;
extern crate regex;
extern crate serde;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod config;
mod display;
mod git;
mod github;
mod types;

use git2::Repository;
use regex::Regex;
use std::env;
use std::fs::File;
use std::path::Path;

use git::discover_repo;

use clap::{App, Arg};
use config::read_config_file;
use display::PearsDisplay;
use github::{GitHubMockAPI, GithubAPI};
use types::ConfigRepo;

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

    let cwd = env::current_dir().expect("Could not get current dir.");
    let config_repo = discover_repo(cwd).expect("Could not determine repo details.");
    println!("{:?}", config_repo);

    let display = PearsDisplay::new();

    let api = GitHubMockAPI {};

    let repo = api.fetch_repo(&config_repo).expect("Could not reach GitHub API.");
    let mut prs = repo.pull_requests.as_vec();
    prs.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    for pr in prs {
        display.pr(pr);
    }
}
