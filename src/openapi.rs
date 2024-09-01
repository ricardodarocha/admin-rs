use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

pub fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}