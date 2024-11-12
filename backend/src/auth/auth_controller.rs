use crate::auth::auth_dto::{LoginRequest, LoginResponse, RegisterRequest, UpdatePasswordRequest};
use crate::auth::auth_service::AuthService;
use actix_web::cookie::{time::Duration, Cookie, SameSite};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

pub async fn login(req: web::Json<LoginRequest>, pool: web::Data<Pool>) -> impl Responder {
  let client = pool.get().await.expect("Failed to get client from pool");
  let client = &**client;

  match AuthService::login(&req.email, &req.password, client).await {
    Ok(login_response) => {
      if let Some(token) = &login_response.token {
        let cookie = Cookie::build("token", token.clone())
          .max_age(Duration::days(365))
          .domain("localhost")
          .path("/")
          .http_only(true)
          .secure(true)
          .same_site(SameSite::Strict)
          .finish();
        HttpResponse::Ok().cookie(cookie).json(login_response)
      } else {
        HttpResponse::Unauthorized().body("Token generation failed")
      }
    },
    Err(_) => HttpResponse::Unauthorized().body("Login failed"),
  }
}

pub async fn register(body: web::Json<RegisterRequest>, pool: web::Data<Pool>) -> impl Responder {
  let client = pool.get().await.expect("Failed to get client from pool");
  let client = &**client;

  match AuthService::register(body.into_inner(), client).await {
    Ok(response) => HttpResponse::Created().json(response),
    Err(_) => HttpResponse::BadRequest().body("Registration failed"),
  }
}

pub async fn update_password(
  body: web::Json<UpdatePasswordRequest>,
  pool: web::Data<Pool>,
) -> impl Responder {
  let client = pool.get().await.expect("Failed to get client from pool");
  let client = &**client;

  match AuthService::update_password(body.into_inner(), client).await {
    Ok(_) => HttpResponse::Ok().body("Password updated successfully"),
    Err(_) => HttpResponse::BadRequest().body("Password update failed"),
  }
}
