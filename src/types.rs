use chrono::{DateTime, Utc};
use serde::{Deserializer, Deserialize};

#[derive(Debug, Clone)]
pub struct GitHubError {
    pub details: String,
}

#[derive(Deserialize, Debug)]
pub struct ConfigRepo {
    pub owner: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub me: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubGraphQLResponse {
    pub data: GitHubGraphQLRepoResponse,
}

#[derive(Deserialize, Debug)]
pub struct GitHubGraphQLRepoResponse {
    pub repository: GitHubRepo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepo {
    pub name: String,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub pull_requests: Vec<GitHubPullRequest>,
}

#[derive(Deserialize, Debug)]
pub struct GitHubUser {
    pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubCommit {
    pub sha: String,
    pub user: GitHubUser,
    pub repo: GitHubRepo,
}

#[derive(Deserialize, Debug)]
pub struct GitHubLabel {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitHubPullRequest {
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

    pub author: GitHubUser,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub labels: Vec<GitHubLabel>,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub comments: Vec<GitHubComment>,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub reviews: Vec<GitHubReview>,
}

impl GitHubPullRequest {
    pub fn is_approved(&self) -> bool {
        self.reviews.iter().any(|e| e.state == "APPROVED")
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitHubReview {
    pub author: GitHubUser,
    pub body_text: String,
    pub state: String,

    #[serde(deserialize_with = "deserialize_pagination")]
    pub comments: Vec<GitHubComment>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitHubComment {
    pub author: GitHubUser,
    pub body_text: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn deserialize_pagination<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where D: Deserializer<'de>,
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

    GraphqlPagination::deserialize(deserializer).map(|p|
        p.edges.into_iter().map(|e| e.node).collect()
    )
}
