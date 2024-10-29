use crate::auth::auth_service::AuthService;
use crate::auth::auth_dto::{LoginRequest, RegisterRequest};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::Pool;

pub async fn login(req: web::Json<LoginRequest>, pool: web::Data<Pool>) -> impl Responder {
  let client = pool.get().await.expect("Failed to get client from pool");
  let client = &**client; // Access the underlying `tokio_postgres::Client`

  match AuthService::login(&req.email, &req.password, client).await {
    Ok(response) => response,
    Err(_) => HttpResponse::Unauthorized().body("Login failed"),
  }
}

pub async fn register(
  req: HttpRequest,
  body: web::Json<RegisterRequest>,
  pool: web::Data<Pool>,
) -> impl Responder {
  let client = pool.get().await.expect("Failed to get client from pool");
  let client = &**client; // Access the underlying `tokio_postgres::Client`

  match AuthService::register(req, body.into_inner(), client).await {
    Ok(response) => response,
    Err(_) => HttpResponse::BadRequest().body("Registration failed"),
  }
}
