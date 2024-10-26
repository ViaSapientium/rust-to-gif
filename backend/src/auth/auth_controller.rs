use crate::auth::auth_service::AuthService;
use crate::auth::dto::{LoginRequest, RegisterRequest};
use crate::common::responses::ApiResponse;
use actix_web::{web, Responder};
use deadpool_postgres::Pool;

pub async fn register(body: web::Json<RegisterRequest>, pool: web::Data<Pool>) -> impl Responder {
  let client = pool.get().await.expect("Error connecting with database");

  match AuthService::register(body.into_inner(), &client).await {
    Ok(response) => response,
    Err(err) => ApiResponse::from_error(err),
  }
}

pub async fn login(body: web::Json<LoginRequest>, pool: web::Data<Pool>) -> impl Responder {
  let client = pool.get().await.expect("Error connecting with database");

  match AuthService::login(&body.email, &body.password, &client).await {
    Ok(response) => response,
    Err(err) => ApiResponse::from_error(err),
  }
}
