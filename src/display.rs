use types;

use console::{Color, Style, Term, Attribute};

pub struct PearsDisplay {
    term: Term,
    width: usize,
}

// fn ago(timestamp_str: String) {
//     let timestamp = DateTime::parse_from_str(timestamp_str);
//     let now = Local::now();
// }

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
        let url_style = Style::new().attr(Attribute::Dim);
        let line = format!(
            "   [#{}] {}\n   Last updated {}\n   {}\n",
            pr.number, pr.title, pr.updated_at, url_style.apply_to(pr.url)
        );
        self.term.write_line(line.as_str()).unwrap();
    }

}
