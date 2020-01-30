use types;

use chrono::prelude::*;
use console::{Attribute, Style, Term, Color};
use std::io;

pub struct PearsDisplay {
    term: Term,
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
        noun = if hours > 1 { "hours" } else { "hour" };
    } else if minutes > 0 {
        count = minutes;
        noun = if minutes > 1 { "minutes" } else { "minute" };
    } else {
        count = seconds;
        noun = if seconds > 1 { "seconds" } else { "second" };
    }

    format!("{} {}", count, noun)
}

impl PearsDisplay {
    pub fn new() -> PearsDisplay {
        let term = Term::stdout();

        PearsDisplay {
            term,
        }
    }

    pub fn repo(&self, repo: &types::ConfigRepo) {
        let repo_style = Style::new().bg(Color::White).fg(Color::Black);
        let line = format!("{:80}\n", repo_style.apply_to(&repo.name));
        self.term.write_line(line.as_str()).unwrap();
    }

    pub fn list(&self, prs: Vec<&types::PullRequest>) {
        let url_style = Style::new().attr(Attribute::Dim);
        let number_style = Style::new().green();
        let label_style = Style::new().cyan();

        for pr in prs {
            let label_str = pr.labels.iter().map(|l| format!("[{}]", l.name)).collect::<Vec<String>>().join(" ");

            let approved = if pr.is_approved() { "✅ " } else { "   " };
            let line = format!(
                "{}{} {} {}\n   Opened by {} | Updated {} ago\n   {}\n",
                approved,
                number_style.apply_to(format!("#{}", pr.number)),
                pr.title,
                label_style.apply_to(label_str),
                pr.author.login,
                ago(pr.updated_at),
                url_style.apply_to(&pr.url)
            );
            self.term.write_line(line.as_str()).unwrap();
        }
    }

    pub fn show(&self, pr: types::PullRequest) -> io::Result<()> {
        let url_style = Style::new().attr(Attribute::Dim);
        let number_style = Style::new().green();
        let label_style = Style::new().cyan();

        let label_str = pr.labels.iter().map(|l| format!("[{}]", l.name)).collect::<Vec<String>>().join(" ");
        let line = format!(
            "{} {} {}\nOpened by {} | Updated {} ago\n{}\n",
            number_style.apply_to(format!("#{}", pr.number)),
            pr.title,
            label_style.apply_to(label_str),
            pr.author.login,
            ago(pr.updated_at),
            url_style.apply_to(pr.url)
        );
        self.term.write_line(line.as_str()).unwrap();

        if let Some(body) = pr.body {
            self.term.write_line("--------------------")?;
            self.term
                .write_line(body.as_str())?;
            self.term.write_line("--------------------\n")?;
        }

        let mut comments = pr.comments;
        comments.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));

        for comment in comments {
            let line = format!(
                "{}, {} ago\n   {}\n",
                comment.author.login,
                ago(comment.created_at),
                &comment.body_text
            );
            self.term.write_line(line.as_str())?;
        }

        Ok(())
    }
}
