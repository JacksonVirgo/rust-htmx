use actix_web::{Responder, HttpResponse, get, post, web};
use actix_files as fs;
use askama::Template;

use crate::views::pages::MainPageTemplate;
use crate::views::partials::ResponseExampleTemplate;

#[get("/")]
async fn load_main_page() -> impl Responder {
    let page = MainPageTemplate {
        data: "Data added via Rust.".to_string()
    };

    let rendered_page = page.render();
    match rendered_page {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Cannot load page")
        }
    }
}

#[post("/clicked")]
async fn load_clicked_button_response() -> impl Responder {
    let page = ResponseExampleTemplate {
        response: "Button Clicked.".to_string()
    };

    let rendered_component = page.render();
    match rendered_component {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Cannot load component")
        }
    }
}

pub fn config_general_routes(config: &mut web::ServiceConfig) {
    config
        .service(load_main_page)
        .service(load_clicked_button_response)
        .service(fs::Files::new("/public", "./public").show_files_listing());
}