use crate::utils::password_generator::generate_password;
use crate::utils::password_validator::validate_password;
use actix_web::{web, HttpResponse, Responder};

pub async fn generate_password_handler() -> impl Responder {
  let password = generate_password();
  HttpResponse::Ok().json(password)
}

pub async fn validate_password_handler(password: web::Json<String>) -> impl Responder {
  let is_valid = validate_password(&password);
  HttpResponse::Ok().json(serde_json::json!({ "is_valid": is_valid }))
}
