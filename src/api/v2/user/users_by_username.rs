use serde::Deserialize;

const URL: &str = "https://api.twitter.com/2/users/by/username/:username";

pub fn url(username: &str) -> String {
    URL.replace(":username", username)
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
}
