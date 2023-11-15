use actix_web::{post, web, HttpResponse, Responder};

use crate::common::types::{
    app_state::AppState, shortened_url_info::ShortenedUrlInfo,
    url_shortening_request::UrlShorteningRequest,
};

#[post("")]
pub async fn create_short_url(
    data: web::Data<AppState>,
    body: web::Json<UrlShorteningRequest>,
) -> impl Responder {
    let redis_client = data.redis_client.clone();

    let shortened_url_info = ShortenedUrlInfo::new(&body.url);
    let key = shortened_url_info.key.clone();

    redis_client
        .set_json(key.clone(), shortened_url_info)
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("key: {}", key))
}
