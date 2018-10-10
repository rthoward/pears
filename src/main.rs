extern crate chrono;
extern crate clap;
extern crate console;
extern crate git2;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate textwrap;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod config;
mod display;
mod git;
mod github;
mod types;

use clap::{App, Arg, SubCommand};
use config::read_config_file;
use display::PearsDisplay;
use git::{discover_repo, parse_repo_description};
use github::{GitHubGraphqlAPI, GithubAPI};
use std::env;
use types::{Config, ConfigRepo, PearsError};

fn list<T: GithubAPI>(
    config: &Config,
    config_repo: &ConfigRepo,
    api: T,
    display: PearsDisplay,
) -> Result<(), PearsError> {
    let repo = api
        .fetch_repo(config, &config_repo)
        .expect("Could not reach GitHub API.");
    let mut prs = repo.pull_requests;
    prs.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    display.list(prs);
    Ok(())
}

fn show<T: GithubAPI>(
    config: &Config,
    config_repo: &ConfigRepo,
    api: T,
    display: PearsDisplay,
    number: i32,
) -> Result<(), PearsError> {
    let repo = api
        .fetch_repo(config, &config_repo)
        .expect("Could not reach GitHub API.");
    let pr = repo
        .pull_requests
        .into_iter()
        .find(|pr| pr.number == number);

    match pr {
        Some(pr) => {
            display.show(pr).unwrap();
            Ok(())
        }
        None => Err(PearsError {
            details: format!("No active PR found with number {}.", number),
        }),
    }
}

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
        .arg(
            Arg::with_name("repo")
                .short("r")
                .long("repo")
                .help("Specify a repository. Format: <owner>/<repo>")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("list").about("lists active pull requests"))
        .subcommand(
            SubCommand::with_name("show")
                .about("details for a pull request")
                .arg(Arg::with_name("number").required(true)),
        )
        .get_matches();

    let config = read_config_file(matches.value_of("config").unwrap())
        .expect("Could not parse config file.");

    let config_repo = if matches.is_present("repo") {
        parse_repo_description(matches.value_of("repo").unwrap())
    } else {
        let cwd = env::current_dir().expect("Could not get current dir.");
        discover_repo(cwd).expect("Could not determine repo details.")
    };

    let display = PearsDisplay::new();
    let api = GitHubGraphqlAPI {};

    let result = match matches.subcommand() {
        ("show", Some(matches)) => {
            let number = matches
                .value_of("number")
                .map(|n| n.parse::<i32>())
                .unwrap()
                .unwrap();
            show(&config, &config_repo, api, display, number)
        }
        (_, _matches) => list(&config, &config_repo, api, display),
    };

    match result.err() {
        Some(error) => println!("{}", error.details),
        None => (),
    };
}
