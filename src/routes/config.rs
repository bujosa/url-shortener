use actix_web::web;

use super::{create_short_url::create_short_url, redirect::redirect};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/shorten").service(create_short_url))
            .service(web::scope("/{short_url}").service(redirect)),
    );
}
