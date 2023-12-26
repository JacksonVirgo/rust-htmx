use actix_web::{Responder, HttpResponse, get, web};

#[get("/")]
async fn load_main_page() -> impl Responder {
    let html_content = std::fs::read_to_string("src/views/index.html");
    match html_content {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::Ok().body("Hello there.")
        }
    }
}

pub fn config_general_routes(config: &mut web::ServiceConfig) {
    config
        .service(load_main_page);
}