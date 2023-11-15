use actix_cors::Cors;
use actix_web::http::header;

pub fn get_cors() -> Cors {
    Cors::default()
        .allowed_origin_fn(|_, _| true)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .supports_credentials()
}
