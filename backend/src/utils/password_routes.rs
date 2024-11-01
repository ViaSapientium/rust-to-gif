use crate::utils::password_controller::{generate_password_handler, validate_password_handler};
use actix_web::web;

pub fn configure_password_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/password")
      .route("/generate", web::get().to(generate_password_handler))
      .route("/validate", web::post().to(validate_password_handler)),
  );
}
