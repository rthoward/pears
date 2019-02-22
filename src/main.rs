extern crate chrono;
extern crate clap;
extern crate console;
extern crate git2;
extern crate regex;
extern crate reqwest;
extern crate serde;

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
    config_repos: &Vec<ConfigRepo>,
    api: T,
    display: PearsDisplay,
) -> Result<(), PearsError> {
    for config_repo in config_repos {
        let repo = api
            .fetch_repo(config, &config_repo)
            .expect("Could not reach GitHub API.");
        let mut prs = repo.pull_requests;
        prs.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        display.repo(config_repo);
        display.list(prs);
    }
    Ok(())
}

fn show<T: GithubAPI>(
    config: &Config,
    config_repos: &Vec<ConfigRepo>,
    api: T,
    display: PearsDisplay,
    number: i32,
) -> Result<(), PearsError> {
    for config_repo in config_repos {
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
                return Ok(())
            }
            None => { () }
        }
    }
    Err(PearsError { details: format!("No active PR found with number {}.", number), })
}

fn show_config(config: &Config) -> Result<(), PearsError> {
    println!("{:?}", config);
    Ok(())
}

fn relevant_repos(config: &Config, local_repo: ConfigRepo, group: Option<&str>) -> Result<Vec<ConfigRepo>, PearsError> {
    let config_repos: Vec<ConfigRepo> = match &config.groups {
        Some(ref groups) => {
            match group {
                Some(group_name) => {
                    let group = groups.iter().find(|&g| g.name == group_name).expect("Could not find group with that name. Please check your config.");
                    group.repos.clone()
                }
                None => vec![local_repo]
            }
        }
        None => { vec![local_repo] }
    };
    Ok(config_repos)
}

fn main() {
    let matches = App::new("pears")
        .version("1.2.1")
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
        .subcommand(
            SubCommand::with_name("list")
            .about("lists active pull requests")
            .arg(Arg::with_name("group").required(false).index(1))
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("details for a pull request")
                .arg(Arg::with_name("number").required(true))
                .arg(Arg::with_name("group").required(false).index(2)),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Show config")
        )
        .get_matches();

    let config = read_config_file(matches.value_of("config").unwrap())
        .expect("Could not parse config file.");

    let local_repo = if matches.is_present("repo") {
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
            let group = matches.value_of("group");
            let repos = relevant_repos(&config, local_repo, group).unwrap();
            show(&config, &repos, api, display, number)
        }
        ("config", _matches) => { show_config(&config) }
        (_, Some(matches)) => {
            let group = matches.value_of("group");
            let repos = relevant_repos(&config, local_repo, group).unwrap();
            list(&config, &repos, api, display)
        }
        (_, None) => {
            let repos = relevant_repos(&config, local_repo, None).unwrap();
            list(&config, &repos, api, display)
        }
    };

    match result.err() {
        Some(error) => println!("{}", error.details),
        None => (),
    };
}
