mod config;
use crate::routes::general_router;
use actix_web::{App, HttpServer};

pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();
    println!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(|| {
        App::new()
            .configure(general_router::config_general_routes)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}