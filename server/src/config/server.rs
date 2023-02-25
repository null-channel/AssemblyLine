use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    #[serde(default)]
    github: Vec<GithubConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubConfig {
    github_private_key: String,
    github_private_pem_file: String,
    webhook_secret: String,
    webhook_uri: String,
}

impl GithubConfig {
    pub fn get_private_key(&self) -> String {
        "do encoding here".into()
    }
}
