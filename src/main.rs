extern crate clap;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

mod config;
mod types;

use reqwest::Url;
use clap::{App, Arg};
use config::read_config_file;
use types::Repo;

fn fetch_repo(repo: &Repo) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls",
        repo.owner, repo.name
    );
    let uri = Url::parse(&url).expect("Could not parse url.");
    let response_text = reqwest::get(uri)
        .expect("Request failed.")
        .text()
        .expect("Could not get json");
    Ok(response_text)
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
        .get_matches();
    let config = read_config_file(matches.value_of("config").unwrap())
        .expect("Could not parse config file.");

    let repo = &config.repos[0];
    let r = fetch_repo(repo);
    println!("{}", r.unwrap());
}
