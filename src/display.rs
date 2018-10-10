use types;

use chrono::prelude::*;
use console::{Attribute, Style, Term};
use std::cmp::min;
use std::io;
use textwrap::fill;

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
        let (_, width) = term.size();

        PearsDisplay {
            term,
            width: width as usize,
        }
    }

    pub fn list(&self, prs: Vec<types::PullRequest>) {
        let url_style = Style::new().attr(Attribute::Dim);

        for pr in prs {
            let approved = if pr.is_approved() { "✅ " } else { "   " };
            let line = format!(
                "{}[#{}] {}\n   Updated {} ago\n   {}\n",
                approved,
                pr.number,
                pr.title,
                ago(pr.updated_at),
                url_style.apply_to(pr.url)
            );
            self.term.write_line(line.as_str()).unwrap();
        }
    }

    pub fn show(&self, pr: types::PullRequest) -> io::Result<()> {
        let url_style = Style::new().attr(Attribute::Dim);
        let paragraph_width = min(self.width, 80);

        let approved = if pr.is_approved() { "✅ " } else { " " };
        let line = format!(
            "[#{}] {}{}\nUpdated {} ago\n{}\n",
            pr.number,
            pr.title,
            approved,
            ago(pr.updated_at),
            url_style.apply_to(pr.url)
        );
        self.term.write_line(line.as_str())?;

        if let Some(body) = pr.body {
            self.term.write_line("--------------------")?;
            self.term
                .write_line(fill(&body, paragraph_width).as_str())?;
            self.term.write_line("--------------------\n")?;
        }

        let mut comments = pr.comments;
        comments.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));

        for comment in comments {
            let line = format!(
                "{}, {} ago\n   {}\n",
                comment.author.login,
                ago(comment.created_at),
                fill(&comment.body_text, paragraph_width)
            );
            self.term.write_line(line.as_str())?;
        }

        Ok(())
    }
}
