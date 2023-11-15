use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use url_shortener::{
    common::{redis::redis::RedisClient, types::app_state::AppState},
    config::cors::get_cors,
    routes::config::config,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let client = RedisClient::new();

    let app_state = AppState {
        redis_client: client.clone().into(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .configure(config)
            .wrap(get_cors())
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")
    .expect("Can not bind to port 8080")
    .run()
    .await
}
