use crate::models::Twitter;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, env};

#[derive(Debug, PartialEq)]
pub enum ApiVersion {
    V1,
    V2,
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
}

pub async fn request<T: DeserializeOwned + std::fmt::Debug>(
    endpoint: &str,
    params: &HashMap<&str, &str>,
    api_version: ApiVersion,
    method: Method,
) -> Result<T, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();

    // if ApiVersion::V1
    if api_version == ApiVersion::V1 {
        let twitter = Twitter::new(
            env::var("TWITTER_CONSUMER_API_KEY").expect("TWITTER_CONSUMER_API_KEY not found"),
            env::var("TWITTER_CONSUMER_API_KEY_SECRET")
                .expect("TWITTER_CONSUMER_API_KEY_SECRET not found"),
            env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN not found"),
            env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET not found"),
        );

        let oauth_header = twitter.create_oauth_header(&method, endpoint, &params);

        headers.insert(
            "Authorization",
            oauth_header.parse::<HeaderValue>().unwrap(),
        );
    } else if api_version == ApiVersion::V2 {
        let bearer_token =
            env::var("TWITTER_BEARER_TOKEN").expect("TWITTER_BEARER_TOKEN not found");

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", bearer_token))?,
        );
    }

    headers.insert(
        "Content-Type",
        "application/json".parse::<HeaderValue>().unwrap(),
    );

    let client = Client::new();
    let resp;

    if method == Method::Get {
        resp = client
            .get(endpoint)
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .json::<T>()
            .await?;
    } else {
        resp = client
            .post(endpoint)
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .json::<T>()
            .await?;
    }

    Ok(resp)
}

pub async fn request_text(
    endpoint: &str,
    params: HashMap<&str, &str>,
    api_version: ApiVersion,
    method: Method,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();

    // if ApiVersion::V1
    if api_version == ApiVersion::V1 {
        let twitter = Twitter::new(
            env::var("TWITTER_CONSUMER_API_KEY").expect("TWITTER_CONSUMER_API_KEY not found"),
            env::var("TWITTER_CONSUMER_API_KEY_SECRET")
                .expect("TWITTER_CONSUMER_API_KEY_SECRET not found"),
            env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN not found"),
            env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET not found"),
        );

        let oauth_header = twitter.create_oauth_header(&method, endpoint, &params);

        headers.insert(
            "Authorization",
            oauth_header.parse::<HeaderValue>().unwrap(),
        );
    } else {
        let bearer_token =
            env::var("TWITTER_BEARER_TOKEN").expect("TWITTER_BEARER_TOKEN not found");

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", bearer_token))?,
        );
    }

    headers.insert(
        "Content-Type",
        "application/json".parse::<HeaderValue>().unwrap(),
    );

    let client = Client::new();
    let resp;

    // params to query string
    let mut query_string = String::new();
    params.iter().for_each(|(k, v)| {
        query_string.push_str(&format!("{}={}&", k, v));
    });

    if method == Method::Get {
        resp = client
            .get(format!("{}?{}", endpoint, query_string))
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .text()
            .await?;
    } else {
        resp = client
            .post(endpoint)
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .text()
            .await?;
    }

    Ok(resp)
}
