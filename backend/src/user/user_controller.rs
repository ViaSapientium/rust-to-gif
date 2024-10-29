use crate::common::responses::ApiResponse;
use crate::user::user_dto::{CreateUserRequest, UpdatePasswordRequest};
use crate::user::user_errors::UserError;
use crate::user::user_service::UserService;
use crate::utils::password::generate_password;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

pub async fn all_users(pool: web::Data<Pool>) -> impl Responder {
  let client = pool.get().await.expect("Error connecting to database");

  match UserService::all_users(&client).await {
    Ok(users) => ApiResponse::success(
      "User list successfully retrieved",
      Some(serde_json::json!({ "users": users })),
    ),
    Err(UserError::DatabaseError(err)) => HttpResponse::InternalServerError().body(err.to_string()),
    Err(_) => HttpResponse::InternalServerError().body("Unknown error occurred"),
  }
}

pub async fn create_user(
  body: web::Json<CreateUserRequest>,
  pool: web::Data<Pool>,
) -> impl Responder {
  let client = pool.get().await.expect("Error connecting to database");

  let password = if body.password.is_empty() {
    generate_password(24)
  } else {
    body.password.clone()
  };

  match UserService::add_user(
    &client,
    &body.username,
    &body.login,
    &body.email,
    Some(&password),
  )
  .await
  {
    Ok(_) => ApiResponse::created(
      "User created successfully",
      Some(serde_json::json!({ "generated_password": password })),
    ),
    Err(UserError::AlreadyExists) => ApiResponse::conflict("User already exists"),
    Err(UserError::DatabaseError(err)) => HttpResponse::InternalServerError().body(err.to_string()),
    Err(_) => HttpResponse::InternalServerError().body("Unknown error occurred"),
  }
}

pub async fn update_password(
  body: web::Json<UpdatePasswordRequest>,
  pool: web::Data<Pool>,
) -> impl Responder {
  let client = pool.get().await.expect("Error connecting to database");

  match UserService::update_password(&client, &body.email, &body.new_password).await {
    Ok(_) => ApiResponse::success("Password updated", None),
    Err(UserError::NotFound) => ApiResponse::not_found("User not found"),
    Err(UserError::DatabaseError(err)) => HttpResponse::InternalServerError().body(err.to_string()),
    Err(_) => HttpResponse::InternalServerError().body("Unknown error occurred"),
  }
}

pub async fn delete_user(req: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
  let login = req.into_inner();
  let client = pool.get().await.expect("Error connecting to database");

  match UserService::delete_user(&client, &login).await {
    Ok(_) => ApiResponse::no_content(),
    Err(UserError::NotFound) => ApiResponse::not_found("User not found"),
    Err(UserError::DatabaseError(err)) => HttpResponse::InternalServerError().body(err.to_string()),
    Err(_) => HttpResponse::InternalServerError().body("Unknown error occurred"),
  }
}
