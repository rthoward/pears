use console::{Color, Style, Term};
use types;

pub struct PearsDisplay {
    term: Term,
}

impl PearsDisplay {
    pub fn new() -> PearsDisplay {
        PearsDisplay {
            term: Term::stdout(),
        }
    }

    pub fn repo(&self, repo: &types::ConfigRepo) {
        let repo_style = Style::new().bg(Color::White).fg(Color::Black);
        let line = format!("{}\n", repo_style.apply_to(&repo.name));
        self.term.write_line(line.as_str()).unwrap();
    }

    pub fn pr(&self, pr: &types::GitHubPullRequest) {
        let line = format!("   {}\n   {}\n", pr.title, pr.html_url);
        self.term.write_line(line.as_str()).unwrap();
    }
}
