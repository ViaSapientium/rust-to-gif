use crate::user::user_controller::{all_users, create_user, delete_user, update_password};
use actix_web::web;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/users")
      .route("/", web::get().to(all_users))
      .route("/", web::post().to(create_user))
      .route("/{login}", web::delete().to(delete_user))
      .route("/password", web::put().to(update_password)),
  );
}
