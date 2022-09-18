const URL: &str = "https://api.twitter.com/1.1/application/rate_limit_status.json";

pub fn url() -> String {
    URL.to_string()
}
