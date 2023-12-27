use askama::Template;

#[derive(Template)]
#[template(path = "response_example.html")]
pub struct ResponseExampleTemplate {
    pub response: String,
}