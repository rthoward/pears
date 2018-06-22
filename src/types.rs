#[derive(Deserialize)]
pub struct Repo {
    owner: String,
    name: String
}

#[derive(Deserialize)]
pub struct Config {
    me: String,
    token: String,
    repos: Vec<Repo>
}
