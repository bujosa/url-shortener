use serde::{Deserialize, Serialize};

use crate::common::functions::url_shortening::hash_url_for_shortening;

#[derive(Serialize, Deserialize)]
pub struct ShortenedUrlInfo {
    pub key: String,
    pub redirect: String,
}

impl ShortenedUrlInfo {
    pub fn new(url: &str) -> ShortenedUrlInfo {
        ShortenedUrlInfo {
            key: hash_url_for_shortening(url),
            redirect: url.to_string(),
        }
    }
}
