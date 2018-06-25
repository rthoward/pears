#[derive(Deserialize)]
pub struct Repo {
    pub owner: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub me: String,
    pub token: String,
    pub repos: Vec<Repo>,
}
