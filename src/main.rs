mod config;
use crate::routes::general_router;
use actix_web::{App, HttpServer};

use config::get_config;

pub mod routes;
pub mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config();
    println!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(|| {
        App::new()
            .configure(general_router::config_general_routes)
    })
    .bind((config.host.clone(), config.port.clone()))?
    .run()
    .await
}