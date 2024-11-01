mod login;
mod register;

use actix_web::web::{ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    login::routes(cfg);
    register::routes(cfg);
}
