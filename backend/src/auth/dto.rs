use crate::user::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
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

#[derive(Serialize)]
pub struct AuthResponse {
  pub success: bool,
  pub token: Option<String>,
  pub user: Option<User>,
  pub message: Option<String>,
}
