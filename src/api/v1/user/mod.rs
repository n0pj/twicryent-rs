use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub id_str: String,
    pub name: Option<String>,
    pub screen_name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    // pub url: Option<String>,
    // pub entities: UserEntities,
    // pub protected: bool,
    // pub followers_count: u64,
    // pub friends_count: u64,
    // pub listed_count: u64,
    // pub created_at: String,
    // pub favourites_count: u64,
    // pub utc_offset: Option<i64>,
    // pub time_zone: Option<String>,
    // pub geo_enabled: bool,
    // pub verified: bool,
    // pub statuses_count: u64,
    // pub lang: Option<String>,
    // pub status: Option<Status>,
    // pub contributors_enabled: bool,
    // pub is_translator: bool,
    // pub is_translation_enabled: bool,
    // pub profile_background_color: String,
    // pub profile_background_image_url: Option<String>,
    pub profile_background_image_url_https: Option<String>,
    // pub profile_background_tile: bool,
    // pub profile_image_url: String,
    pub profile_image_url_https: Option<String>,
    // pub profile_banner_url: Option<String>,
    // pub profile_link_color: String,
    // pub profile_sidebar_border_color: String,
    // pub profile_sidebar_fill_color: String,
    // pub profile_text_color: String,
    // pub profile_use_background_image: bool,
    // pub has_extended_profile: bool,
    // pub default_profile: bool,
    // pub default_profile_image: bool,
    // pub following: Option<bool>,
    // pub live_following: Option<bool>,
    // pub follow_request_sent: Option<bool>,
    // pub notifications: Option<bool>,
    // pub muting: Option<bool>,
    // pub blocking: Option<bool>,
    // pub blocked_by: Option<bool>,
    // pub translator_type: String,
    // pub withheld_in_countries: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserEntities {
    pub url: Option<UserEntity>,
    pub description: Option<UserEntity>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserEntity {
    pub urls: Vec<Url>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Url {
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    pub indices: Vec<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    pub created_at: String,
    pub id: u64,
    pub id_str: String,
    pub text: String,
    pub truncated: bool,
    pub entities: StatusEntities,
    pub source: String,
    pub in_reply_to_status_id: Option<u64>,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_user_id: Option<u64>,
    pub in_reply_to_user_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,
    pub geo: Option<String>,
    pub coordinates: Option<String>,
    pub place: Option<String>,
    pub contributors: Option<String>,
    pub is_quote_status: bool,
    pub retweet_count: u64,
    pub favorite_count: u64,
    pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub lang: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusEntities {
    pub hashtags: Vec<Hashtag>,
    pub symbols: Vec<Symbol>,
    pub user_mentions: Vec<UserMention>,
    pub urls: Vec<Url>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Hashtag {
    pub text: String,
    pub indices: Vec<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Symbol {
    pub text: String,
    pub indices: Vec<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserMention {
    pub screen_name: String,
    pub name: String,
    pub id: u64,
    pub id_str: String,
    pub indices: Vec<u64>,
}

// pub struct UserEntities {
//     pub url: UserUrl,
//     pub description: UserDescription,
// }

pub type Users = Vec<User>;
