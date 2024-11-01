use crate::auth::auth_service::AuthService;
use actix_web::web;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/validate", web::get().to(AuthService::validate_token))
    .route("/login", web::post().to(AuthService::login))
    .route("/register", web::post().to(AuthService::register));
}
