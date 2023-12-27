use actix_web::{Responder, HttpResponse, get, web};
use actix_files as fs;
use askama::Template;
use crate::views::IndexTemplate;

#[get("/")]
async fn load_main_page() -> impl Responder {
    let page = IndexTemplate {
        data: "Data added via Rust.".to_string()
    };

    let rendered_page = page.render();
    match rendered_page {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::Ok().body("Hello there.")
        }
    }
}

pub fn config_general_routes(config: &mut web::ServiceConfig) {
    config
        .service(load_main_page)
        .service(fs::Files::new("/public", "./public").show_files_listing());
}