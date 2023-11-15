use actix_web::{
    web::{self, Redirect},
    Responder,
};

use crate::common::types::{app_state::AppState, shortened_url_info::ShortenedUrlInfo};

pub async fn redirect(data: web::Data<AppState>, short_url: web::Path<String>) -> impl Responder {
    let redis_client = data.redis_client.clone();
    let url: ShortenedUrlInfo = redis_client.get_json(short_url.into_inner()).await.unwrap();

    Redirect::to(url.redirect).permanent()
}
