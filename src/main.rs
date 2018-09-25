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
use github::{GitHubMockAPI, GithubAPI};

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

    for config_repo in config.repos {
        let api = GitHubMockAPI {};

        let repo = api.fetch_repo(&config_repo).expect("Could not reach GitHub API.");
        let mut prs = repo.pullRequests.as_vec();
        prs.sort_by(|a, b| a.updatedAt.cmp(&b.updatedAt));

        if !prs.is_empty() {
            display.repo(&config_repo);
        }

        for pr in prs {
            display.pr(&pr);
        }
    }
}
