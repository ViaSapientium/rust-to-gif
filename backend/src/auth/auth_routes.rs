use actix_web::web;
use crate::auth::auth_service::AuthService;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/validate",
        web::get().to(AuthService::validate_token),
    );
}
