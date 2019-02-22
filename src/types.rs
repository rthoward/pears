use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GitHubError {
    pub details: String,
}

#[derive(Debug, Clone)]
pub struct PearsError {
    pub details: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigRepo {
    pub owner: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub me: String,
    pub token: String,
    pub groups: Option<Vec<Group>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Group {
    pub name: String,
    pub repos: Vec<ConfigRepo>
}

#[derive(Deserialize, Debug)]
pub struct GraphqlResponse {
    pub data: RepoResponse,
}

#[derive(Deserialize, Debug)]
pub struct RepoResponse {
    pub repository: Repo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub name: String,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub pull_requests: Vec<PullRequest>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub sha: String,
    pub user: User,
    pub repo: Repo,
}

#[derive(Deserialize, Debug)]
pub struct Label {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub id: String,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub number: i32,
    pub url: String,
    pub mergeable: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub merged_at: Option<DateTime<Utc>>,

    pub author: User,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub labels: Vec<Label>,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub comments: Vec<Comment>,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub reviews: Vec<Review>,
}

impl PullRequest {
    pub fn is_approved(&self) -> bool {
        self.reviews.iter().any(|e| e.state == "APPROVED")
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub author: User,
    pub body_text: String,
    pub state: String,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub comments: Vec<Comment>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub author: User,
    pub body_text: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn deserialize_pagination<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize, Debug)]
    pub struct GraphqlPagination<T> {
        pub edges: Vec<GraphqlPaginationNode<T>>,
    }

    #[derive(Deserialize, Debug)]
    pub struct GraphqlPaginationNode<T> {
        pub node: T,
    }

    GraphqlPagination::deserialize(deserializer)
        .map(|p| p.edges.into_iter().map(|e| e.node).collect())
}
