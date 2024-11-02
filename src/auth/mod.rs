// pub mod middleware;
pub mod model;
mod login;
mod register;
mod forgot;
mod recover;
mod reset;

use actix_web::web::ServiceConfig;

pub fn routes(cfg: &mut ServiceConfig) {
    login::routes(cfg);
    register::routes(cfg);
    forgot::routes(cfg);
    recover::routes(cfg);
    reset::routes(cfg);
}
