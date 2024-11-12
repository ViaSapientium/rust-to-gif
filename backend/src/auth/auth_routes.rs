use crate::auth::auth_controller::{login, register};
use actix_web::web;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/validate", web::get().to(login))
    .route("/login", web::post().to(login))
    .route("/register", web::post().to(register));
}
