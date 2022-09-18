use crate::http::Method;
use chrono::Utc;
use hmacsha1::hmac_sha1;
use percent_encoding::{utf8_percent_encode, AsciiSet};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

// ReExport
pub use crate::api::*;

const FRAGMENT: &AsciiSet = &percent_encoding::NON_ALPHANUMERIC
    .remove(b'*')
    .remove(b'-')
    .remove(b'_')
    .remove(b'.');

#[derive(Debug)]
pub struct Twitter {
    consumer_api_key: String,
    consumer_api_key_secret: String,
    access_token: String,
    access_token_secret: String,
}

/// Twitter OAuth 1.0 Client
/// Twitter OAuth 2.0 Client
impl Twitter {
    pub fn new(
        consumer_api_key: String,
        consumer_api_key_secret: String,
        access_token: String,
        access_token_secret: String,
    ) -> Self {
        Twitter {
            consumer_api_key,
            consumer_api_key_secret,
            access_token,
            access_token_secret,
        }
    }

    // fn split_query(query: &str) -> HashMap<String, String> {
    //     let mut param = HashMap::new();

    //     for q in query.split('&') {
    //         let (k, v) = q.split_once('=').unwrap();
    //         let _ = param.insert(k.into(), v.into());
    //     }

    //     param
    // }

    // async fn get_request_token(self) -> Self {
    //     let endpoint = "https://api.twitter.com/oauth/request_token";
    //     // let endpoint = format!("{}?oauth_callback=local", endpoint);

    //     let mut params = HashMap::new();

    //     params.insert("oauth_callback", "oob");

    //     let res = http::request_text(&endpoint, params, ApiVersion::V1, Method::Post).await;

    //     println!("res: {:?}", res);

    //     let queries = Twitter::split_query(&res.unwrap());

    //     Self {
    //         consumer_api_key: self.consumer_api_key,
    //         consumer_api_key_secret: self.consumer_api_key_secret,
    //         access_token: queries.get("oauth_token").unwrap().to_string(),
    //         access_token_secret: queries.get("oauth_token_secret").unwrap().to_string(),
    //     }
    // }

    // // deprecated
    // async fn get_authenticate_url(self) -> Result<Self, Box<dyn std::error::Error>> {
    //     let endpoint = "https://api.twitter.com/oauth/authenticate";
    //     let endpoint = format!(
    //         "{}?oauth_token={}&oauth_token_secret={}",
    //         endpoint, self.access_token, self.access_token_secret
    //     );

    //     println!("url: {}", endpoint);

    //     Ok(self)
    // }

    // // deprecated
    // async fn get_access_token(self, input: String) -> Result<Self, Box<dyn std::error::Error>> {
    //     let endpoint = "https://api.twitter.com/oauth/access_token";
    //     let mut headers = HeaderMap::new();
    //     let params = HashMap::new();
    //     let endpoint = format!(
    //         "{}?oauth_verifier={}&oauth_token={}&oauth_consumer_key={}",
    //         endpoint, input, self.access_token, self.consumer_api_key
    //     );

    //     println!("url: {}", endpoint);

    //     let oauth_header = self.create_oauth_header(&Method::Get, &endpoint, &params);

    //     headers.insert(
    //         "Authorization",
    //         oauth_header.parse::<HeaderValue>().unwrap(),
    //     );

    //     headers.insert(
    //         "Content-Type",
    //         "application/json".parse::<HeaderValue>().unwrap(),
    //     );

    //     let client = Client::new();
    //     let res;

    //     res = client
    //         .post(format!("{}", endpoint))
    //         .headers(headers)
    //         .send()
    //         .await?
    //         .text()
    //         .await?;

    //     println!("get_access_token_res: {}", res);

    //     let queries = Twitter::split_query(&res);

    //     Ok(Self {
    //         consumer_api_key: self.consumer_api_key,
    //         consumer_api_key_secret: self.consumer_api_key_secret,
    //         access_token: queries.get("oauth_token").unwrap().to_string(),
    //         access_token_secret: queries.get("oauth_token_secret").unwrap().to_string(),
    //     })
    // }

    // deprecated
    pub fn get_authorize_url(&self) -> String {
        let endpoint = "https://api.twitter.com/oauth/authorize";

        format!("{}?oauth_token={}", endpoint, self.access_token)
    }

    pub async fn get_text(
        &self,
        endpoint: &str,
        params: &HashMap<&str, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        let client = Client::new();
        let oauth_header = self.create_oauth_header(&Method::Get, &endpoint, &params);
        let endpoint = format!("{}?{}", endpoint, self.create_query(&params));

        headers.insert(
            "Authorization",
            oauth_header.parse::<HeaderValue>().unwrap(),
        );

        headers.insert(
            "Content-Type",
            "application/json".parse::<HeaderValue>().unwrap(),
        );

        let res = client
            .get(format!("{}", endpoint))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &HashMap<&str, String>,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        let client = Client::new();
        let oauth_header = self.create_oauth_header(&Method::Get, &endpoint, &params);
        let endpoint = format!("{}?{}", endpoint, self.create_query(&params));

        headers.insert(
            "Authorization",
            oauth_header.parse::<HeaderValue>().unwrap(),
        );

        headers.insert(
            "Content-Type",
            "application/json".parse::<HeaderValue>().unwrap(),
        );

        let res = client
            .get(format!("{}", endpoint))
            .headers(headers)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(res)
    }

    pub async fn post(
        &self,
        endpoint: &str,
        params: HashMap<&str, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        let client = Client::new();
        let oauth_header = self.create_oauth_header(&Method::Post, &endpoint, &params);
        let endpoint = format!("{}?{}", endpoint, self.create_query(&params));

        headers.insert(
            "Authorization",
            oauth_header.parse::<HeaderValue>().unwrap(),
        );

        headers.insert(
            "Content-Type",
            "application/json".parse::<HeaderValue>().unwrap(),
        );

        let res = client
            .post(format!("{}", endpoint))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }

    fn create_query(&self, params: &HashMap<&str, String>) -> String {
        let mut query = String::new();

        for (k, v) in params {
            query.push_str(&format!("{}={}&", k, v));
        }

        query.pop();

        query
    }

    /// oauth_nonce	リクエスト毎に一意の文字列
    /// oauth_timestamp	現在の UNIX 時間
    /// oauth_signature_method "HMAC-SHA1"
    /// oauth_version "1.0"
    /// oauth_consumer_key Consumer Api Key
    /// oauth_token	Access Token
    /// oauth_signature	上記パラメータをもとに計算
    /// 参考: https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
    /// https://scienceboy.jp/88io/2022/01/rust-tweet/#toc7
    fn create_oauth_params(
        &self,
        endpoint: &str,
        method: &Method,
        params: &HashMap<&str, String>,
    ) -> HashMap<&str, String> {
        // create parameter string
        let mut oauth_params = HashMap::new();
        let timestamp = Utc::now().timestamp().to_string();
        let nonce = format!("nonce{}", Utc::now().timestamp().to_string());
        let oauth_signature_method = "HMAC-SHA1".to_string();
        let oauth_version = "1.0".to_string();

        oauth_params.insert("oauth_nonce", nonce);
        oauth_params.insert("oauth_consumer_key", self.consumer_api_key.clone());
        oauth_params.insert("oauth_signature_method", oauth_signature_method);
        oauth_params.insert("oauth_timestamp", timestamp);
        oauth_params.insert("oauth_token", self.access_token.clone());
        oauth_params.insert("oauth_version", oauth_version);

        let mut signature_params = oauth_params
            .clone()
            .into_iter()
            .collect::<Vec<(&str, String)>>();

        for (k, v) in params {
            signature_params.push((k, v.to_string()));
        }

        signature_params.sort();

        let oauth_signature = self.generate_signature(method, endpoint, &signature_params);

        // insert signature to oauth params
        oauth_params.insert("oauth_signature", oauth_signature);

        oauth_params
    }

    fn generate_signature(
        &self,
        method: &Method,
        endpoint: &str,
        params: &Vec<(&str, String)>,
    ) -> String {
        let param_string = params
            .iter()
            .map(|(key, value)| {
                format!(
                    "{}={}",
                    utf8_percent_encode(key, FRAGMENT),
                    utf8_percent_encode(value, FRAGMENT)
                )
            })
            .collect::<Vec<String>>()
            .join("&");

        // calculate oauth_signature
        let signature_key = self.create_signature_key();

        // create signature base string
        let method = match method {
            Method::Get => "GET",
            Method::Post => "POST",
        };
        let signature_base = format!(
            "{}&{}&{}",
            method,
            utf8_percent_encode(endpoint, FRAGMENT),
            utf8_percent_encode(&param_string, FRAGMENT)
        );

        // generate signature
        let hash = hmac_sha1(signature_key.as_bytes(), signature_base.as_bytes());

        base64::encode(hash)
    }

    pub fn create_oauth_header(
        &self,
        method: &Method,
        endpoint: &str,
        params: &HashMap<&str, String>,
    ) -> String {
        let oauth_params = self.create_oauth_params(endpoint, method, params);
        let oauth_params = oauth_params
            .clone()
            .into_iter()
            .collect::<Vec<(&str, String)>>();

        let oauth_header = format!(
            "OAuth {}",
            oauth_params
                .iter()
                .map(|(key, value)| {
                    format!(
                        r#"{}="{}""#,
                        utf8_percent_encode(key, FRAGMENT),
                        utf8_percent_encode(value, FRAGMENT)
                    )
                })
                .collect::<Vec<String>>()
                .join(", ")
        );

        oauth_header
    }

    fn create_signature_key(&self) -> String {
        format!(
            "{}&{}",
            utf8_percent_encode(&self.consumer_api_key_secret, FRAGMENT),
            utf8_percent_encode(&self.access_token_secret, FRAGMENT)
        )
    }
}
