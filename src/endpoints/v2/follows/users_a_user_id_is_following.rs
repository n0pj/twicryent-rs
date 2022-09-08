// use reqwest::header::HeaderValue;
use serde::Deserialize;
// use std::collections::HashMap;

const URL: &str = "https://api.twitter.com/2/users/:id/following";

pub fn url(id: &str) -> String {
    URL.replace(":id", id)
}

// pub fn query<'a>(max_results: u8, pagination_token: &str) -> HashMap<&'a str, &'a str> {
//     let mut query = HashMap::new();
//     query.insert("max_results", &max_results);
//     query.insert("pagination_token", pagination_token);
//     query
// }

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
}
