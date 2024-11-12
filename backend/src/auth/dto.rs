use crate::user::user::User;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RegisterRequest {
  pub login: String,
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(serde::Serialize)]
pub struct AuthResponse {
  pub success: bool,
  pub token: Option<String>,
  pub user: Option<User>,
  pub message: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct UpdatePasswordRequest {
  pub email: String,
  pub new_password: String,
}

#[derive(serde::Deserialize)]
pub struct DeleteUserRequest {
  pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
  pub sub: String,
  pub exp: usize,
}
