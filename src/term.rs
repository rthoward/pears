extern crate console;

use types;

pub fn display_repo(repo: &types::ConfigRepo) {
    println!("{}\n", repo.name);
}

pub fn display_pr(pr: &types::GitHubPullRequest) {
    println!("   {}", pr.title);
    println!("   {}\n", pr.html_url);
}
