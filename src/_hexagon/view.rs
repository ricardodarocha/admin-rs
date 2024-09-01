// auth_views.rs

use actix_web::HttpResponse;
use serde_json::json;

use crate::auth::User;

pub fn login_response(token: String) -> HttpResponse {
    HttpResponse::Ok().json(json!({ "token": token }))
}

pub fn logout_response() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "message": "Successfully logged out" }))
}

pub fn register_response(user: User) -> HttpResponse {
    HttpResponse::Ok().json(user)
}
