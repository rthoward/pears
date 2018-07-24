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
pub struct GitHubRepo {
    pub owner: String,
    pub name: String,
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct GitHubUser {
    pub login: String,
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct GitHubCommit {
    pub sha: String,
    pub user: GitHubUser,
    pub repo: GitHubRepo,
}

#[derive(Deserialize, Debug)]
pub struct GitHubPullRequest {
    pub id: i32,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub number: i32,
    pub url: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,

    pub user: GitHubUser,
}
