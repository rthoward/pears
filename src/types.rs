use chrono::{DateTime, Utc};

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
    pub pull_requests: GraphqlPagination<GitHubPullRequest>,
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
    pub labels: GraphqlPagination<GitHubLabel>,
    pub comments: GraphqlPagination<GitHubComment>,
    pub reviews: GraphqlPagination<GitHubReview>,
}

impl GitHubPullRequest {
    pub fn is_approved(&self) -> bool {
        false
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitHubReview {
    pub author: GitHubUser,
    pub body_text: String,
    pub comments: GraphqlPagination<GitHubComment>,
    pub state: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct GraphqlPagination<T> {
    pub edges: Vec<GraphqlPaginationNode<T>>,
}

impl<T> GraphqlPagination<T> {
    // I can't figure out IntoIter :(
    pub fn as_vec(self) -> Vec<T> {
        self.edges.into_iter().map(|e| e.node).collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct GraphqlPaginationNode<T> {
    pub node: T,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitHubComment {
    pub author: GitHubUser,
    pub body_text: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
