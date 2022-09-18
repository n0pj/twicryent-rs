use serde::Deserialize;

// https://developer.twitter.com/en/docs/twitter-api/v1/tweets/timelines/api-reference/get-statuses-user_timeline
const URL: &str = "https://api.twitter.com/1.1/statuses/user_timeline.json";

pub fn url() -> String {
    URL.to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserTimeline {
    pub created_at: String,
    pub id: u64,
    pub id_str: String,
    pub text: String,
    // pub truncated: bool,
    // pub entities: Entities,
    pub extended_entities: Option<ExtendedEntities>,
    // pub source: String,
    // pub in_reply_to_status_id: Option<u64>,
    // pub in_reply_to_status_id_str: Option<String>,
    // pub in_reply_to_user_id: Option<u64>,
    // pub in_reply_to_user_id_str: Option<String>,
    // pub in_reply_to_screen_name: Option<String>,
    // pub user: User,
    // pub geo: Option<String>,
    // pub coordinates: Option<String>,
    // pub place: Option<String>,
    // pub contributors: Option<String>,
    // pub is_quote_status: bool,
    // pub retweet_count: u64,
    // pub favorite_count: u64,
    // pub favorited: bool,
    pub retweeted: bool,
    // pub possibly_sensitive: Option<bool>,
    // pub lang: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Entities {
    pub hashtags: Vec<Hashtag>,
    pub symbols: Vec<Symbol>,
    pub user_mentions: Vec<UserMention>,
    pub urls: Vec<Url>,
    pub media: Option<Vec<Media>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Hashtag {
    pub text: String,
    pub indices: Vec<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Symbol {
    pub text: String,
    pub indices: Vec<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserMention {
    pub screen_name: String,
    pub name: String,
    pub id: u64,
    pub id_str: String,
    pub indices: Vec<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Url {
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    pub indices: Vec<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Media {
    pub id: u64,
    pub id_str: String,
    pub indices: Vec<u64>,
    pub media_url: String,
    pub media_url_https: String,
    pub url: String,
    pub display_url: String,
    pub expanded_url: String,
    #[serde(rename = "type")]
    pub kind: String,
    // pub r#type: String,
    pub sizes: Sizes,
    pub video_info: Option<VideoInfo>,
    // pub additional_media_info: Option<AdditionalMediaInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VideoInfo {
    pub aspect_ratio: Vec<u64>,
    pub duration_millis: u64,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Variant {
    pub bitrate: Option<u64>,
    pub content_type: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Sizes {
    pub thumb: Size,
    pub small: Size,
    pub large: Size,
    pub medium: Size,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Size {
    pub w: u64,
    pub h: u64,
    pub resize: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExtendedEntities {
    pub media: Vec<Media>,
}

pub type UserTimelines = Vec<UserTimeline>;
