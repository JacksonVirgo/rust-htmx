use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct MainPageTemplate {
    pub data: String,
}