#[derive(Deserialize, Debug)]
pub struct ConfigRepo {
    pub owner: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub me: String,
    pub token: String,
    pub repos: Vec<ConfigRepo>,
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
pub struct GitHubRepo {
    pub name: String,
    pub pullRequests: GraphqlPagination<GitHubPullRequest>,
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
    name: String
}

#[derive(Deserialize, Debug)]
pub struct GitHubPullRequest {
    pub id: String,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub number: i32,
    pub labels: GraphqlPagination<GitHubLabel>,
    pub url: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub closedAt: Option<String>,
    pub mergedAt: Option<String>,
    pub author: GitHubUser,
    pub comments: GraphqlPagination<GitHubComment>,
    pub reviews: GraphqlPagination<GitHubReview>
}

#[derive(Deserialize, Debug)]
pub struct GitHubReview {
    pub createdAt: String,
    pub updatedAt: String,
    pub author: GitHubUser,
    pub bodyText: String,
    pub comments: GraphqlPagination<GitHubComment>
}

#[derive(Deserialize, Debug)]
pub struct GraphqlPagination<T> {
    pub edges: Vec<GraphqlPaginationNode<T>>,
}

impl<T> GraphqlPagination<T> {
    // I can't figure out IntoIter
    pub fn as_vec(self) -> Vec<T> {
        self.edges.into_iter().map(|e| e.node).collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct GraphqlPaginationNode<T> {
    pub node: T
}

#[derive(Deserialize, Debug)]
pub struct GitHubComment {
    pub createdAt: String,
    pub updatedAt: String,
    pub author: GitHubUser,
    pub bodyText: String
}
