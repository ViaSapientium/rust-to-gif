use crate::auth::dto::UpdatePasswordRequest;
use crate::common::errors::UserServiceError;
use crate::common::responses::ApiResponse;
use crate::user::user::{User, UserMethods};
use crate::user::user_service::UserService;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

pub async fn all_users(pool: web::Data<Pool>) -> impl Responder {
  let client = pool.get().await.expect("Error connecting with database");

  match User::all(&client).await {
    Ok(users) => ApiResponse::success(
      "User list successfully retrieved",
      Some(serde_json::json!({ "users": users })),
    ),
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}

pub async fn create_user(body: web::Json<User>, pool: web::Data<Pool>) -> impl Responder {
  let client = pool.get().await.expect("Error connecting with database");

  match UserService::add_user(
    &client,
    &body.username,
    &body.login,
    &body.email,
    &body.password,
  )
  .await
  {
    Ok(_) => ApiResponse::created("User created successfully", None),
    Err(UserServiceError::HashingError(err)) => {
      HttpResponse::InternalServerError().body(err.to_string())
    }
    Err(UserServiceError::DatabaseError(err)) => {
      HttpResponse::InternalServerError().body(err.to_string())
    }
  }
}

pub async fn update_password(
  body: web::Json<UpdatePasswordRequest>,
  pool: web::Data<Pool>,
) -> impl Responder {
  let client = pool.get().await.expect("Error connecting with database");

  match UserService::update_password(&client, &body.email, &body.new_password).await {
    Ok(_) => ApiResponse::success("Password updated", None),
    Err(UserServiceError::HashingError(err)) => {
      HttpResponse::InternalServerError().body(err.to_string())
    }
    Err(UserServiceError::DatabaseError(err)) => {
      HttpResponse::InternalServerError().body(err.to_string())
    }
  }
}

pub async fn delete_user(req: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
  let login = req.into_inner();
  let client = pool.get().await.expect("Error connecting with database");

  match UserService::delete_user(&client, &login).await {
    Ok(_) => ApiResponse::no_content(),
    Err(UserServiceError::HashingError(err)) => {
      HttpResponse::InternalServerError().body(err.to_string())
    }
    Err(UserServiceError::DatabaseError(err)) => {
      HttpResponse::InternalServerError().body(err.to_string())
    }
  }
}
