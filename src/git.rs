use git2::Repository;
use regex::Regex;
use std::path::PathBuf;

use types::ConfigRepo;

pub fn discover_repo(path: PathBuf) -> Option<ConfigRepo> {
    let repo = Repository::discover(path).expect("Couldn't find git repo.");
    let origin = repo.find_remote("origin")
        .expect("Could't find origin remote.");
    let origin_url = origin.url().expect("No URL for origin remote?");
    let re = Regex::new(r"github.com[/:](?P<owner>.*)/(?P<name>.*)\.git").unwrap();
    let captures = re.captures(origin_url).expect("Could not parse repo url.");

    Some(ConfigRepo {
        owner: String::from(&captures["owner"]),
        name: String::from(&captures["name"]),
    })
}

pub fn parse_repo_description(description: &str) -> ConfigRepo {
    let re = Regex::new(r"(?P<owner>.*)/(?P<name>.*)").unwrap();
    let captures = re.captures(description)
        .expect("Could not parse repo description.");
    ConfigRepo {
        owner: String::from(&captures["owner"]),
        name: String::from(&captures["name"]),
    }
}
