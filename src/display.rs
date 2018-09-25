use console::{Color, Style, Term};
use types;

pub struct PearsDisplay {
    term: Term,
    width: usize,
}

impl PearsDisplay {
    pub fn new() -> PearsDisplay {
        let term = Term::stdout();
        let (_, width) = term.size();

        PearsDisplay {
            term,
            width: width as usize,
        }
    }

    pub fn repo(&self, repo: &types::ConfigRepo) {
        let repo_style = Style::new().bg(Color::White).fg(Color::Black);
        let line = format!(
            "{:width$}\n",
            repo_style.apply_to(&repo.name),
            width = self.width
        );
        self.term.write_line(line.as_str()).unwrap();
    }

    pub fn pr(&self, pr: types::GitHubPullRequest) {
        let line = format!(
            "   [#{}] {}\n   Last updated {}\n   {}\n",
            pr.number, pr.title, pr.updatedAt, pr.url
        );
        self.term.write_line(line.as_str()).unwrap();
    }
}
