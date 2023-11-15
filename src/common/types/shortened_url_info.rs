use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShortenedUrlInfo {
    pub key: String,
    pub url: String,
    pub redirect: String,
}
