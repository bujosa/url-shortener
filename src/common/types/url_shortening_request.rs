use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UrlShorteningRequest {
    pub url: String,
}
