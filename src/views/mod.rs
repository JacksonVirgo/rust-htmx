use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub data: String,
}

#[derive(Template)]
#[template(path = "response_example.html")]
pub struct ResponseExampleTemplate {
    pub response: String,
}