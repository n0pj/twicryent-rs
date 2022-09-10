use crate::api::v1::user::Users;
use serde::Deserialize;

// https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/follow-search-get-users/api-reference/get-friends-list
const URL: &str = "https://api.twitter.com/1.1/friends/list.json";

pub fn url() -> String {
    URL.to_string()
}

// "users": [
// ],
// "next_cursor": 88707,
// "next_cursor_str": "",
// "previous_cursor": 0,
// "previous_cursor_str": "0",
// "total_count": null
#[derive(Deserialize, Debug)]
pub struct List {
    pub users: Users,
    pub next_cursor: Option<i64>,
    pub next_cursor_str: String,
    pub previous_cursor: Option<i64>,
    pub previous_cursor_str: String,
    pub total_count: Option<i64>,
}
