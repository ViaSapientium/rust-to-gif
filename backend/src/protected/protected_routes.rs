use crate::auth::auth_service::AuthService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn protected_route(req: HttpRequest) -> impl Responder {
  let response = AuthService::validate_token(req).await;
  if response.status().is_success() {
    HttpResponse::Ok().body("Access granted to protected route")
  } else {
    HttpResponse::Unauthorized().body("Access denied")
  }
}

pub fn configure_protected_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/protected", web::get().to(protected_route));
}
