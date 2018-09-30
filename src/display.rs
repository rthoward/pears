use types;

use chrono::prelude::*;
use console::{Attribute, Color, Style, Term};

pub struct PearsDisplay {
    term: Term,
    width: usize,
}

fn ago(timestamp: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now - timestamp;

    let days = duration.num_days();
    let hours = duration.num_hours();
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds();

    let noun: &str;
    let count: i64;

    if days > 0 {
        count = days;
        noun = if days > 1 { "days" } else { "day" };
    } else if hours > 0 {
        count = hours;
        noun = if days > 1 { "hours" } else { "hour" };
    } else if minutes > 0 {
        count = minutes;
        noun = if days > 1 { "minutes" } else { "minute" };
    } else {
        count = seconds;
        noun = if days > 1 { "seconds" } else { "second" };
    }

    format!("{} {}", count, noun)
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
        let url_style = Style::new().attr(Attribute::Dim);
        let approved = if pr.is_approved() { "✅" } else { "" };

        let line = format!(
            "{}   [#{}] {}\n   Updated {} ago\n   {}\n",
            approved,
            pr.number,
            pr.title,
            ago(pr.updated_at),
            url_style.apply_to(pr.url)
        );
        self.term.write_line(line.as_str()).unwrap();
    }
}
