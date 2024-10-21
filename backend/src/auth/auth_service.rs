use crate::auth::dto::RegisterRequest;
use crate::auth::jwt::{generate_jwt, validate_jwt};
use crate::common::responses::ApiResponse;
use crate::user::user::{User, UserMethods};
use actix_web::{HttpRequest, HttpResponse};
use argon2::{
  self,
  password_hash::{PasswordHash, SaltString},
  Argon2, PasswordHasher, PasswordVerifier,
};
use serde_json::json;
use thiserror::Error;
use tokio_postgres::GenericClient;

#[derive(Error, Debug)]
pub enum AuthServiceError {
  #[error("Hashing error")]
  HashingError(String),
  #[error("Database error")]
  DatabaseError(#[from] tokio_postgres::Error),
}

impl From<argon2::password_hash::Error> for AuthServiceError {
  fn from(err: argon2::password_hash::Error) -> Self {
    AuthServiceError::HashingError(err.to_string())
  }
}

impl From<AuthServiceError> for actix_web::Error {
  fn from(err: AuthServiceError) -> Self {
    actix_web::error::ErrorInternalServerError(err)
  }
}

pub struct AuthService;

impl AuthService {
  pub async fn login(
    email: &str,
    password: &str,
    client: &(impl GenericClient + Sync),
  ) -> Result<HttpResponse, AuthServiceError> {
    if let Some(user) = User::find_by_login_or_email(client, email, email).await? {
      if !Self::verify_password(&user.password, password) {
        return Ok(ApiResponse::unauthorized("Mot de passe incorrect"));
      }
      let token = generate_jwt(&user.email);
      return Ok(ApiResponse::success(
        "Connexion réussie",
        Some(json!({ "user": user, "token": token })),
      ));
    }
    Ok(ApiResponse::not_found("Utilisateur non trouvé"))
  }

  pub async fn register(
    _req: HttpRequest,
    body: RegisterRequest,
    client: &(impl GenericClient + Sync),
  ) -> Result<HttpResponse, AuthServiceError> {
    let email = &body.email;
    if let Some(_) = User::find_by_login_or_email(client, email, email).await? {
      return Ok(ApiResponse::conflict("L'utilisateur existe déjà"));
    }
    let hashed_password = Self::hash_password(&body.password)?;
    User::add_user(client, &body.username, email, &hashed_password).await?;
    let token = generate_jwt(email);
    Ok(ApiResponse::success(
      "Inscription réussie",
      Some(json!({ "user": body, "token": token })),
    ))
  }

  pub async fn validate_token(req: HttpRequest) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
      if let Ok(auth_str) = auth_header.to_str() {
        if auth_str.starts_with("Bearer ") {
          let token = &auth_str[7..];
          if validate_jwt(token) {
            return ApiResponse::success("Token valide", None);
          }
        }
      }
    }
    ApiResponse::unauthorized("Token invalide")
  }

  fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
      Ok(hash) => hash,
      Err(_) => return false,
    };
    let argon2 = Argon2::default();
    argon2
      .verify_password(password.as_bytes(), &parsed_hash)
      .is_ok()
  }

  fn hash_password(password: &str) -> Result<String, AuthServiceError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
  }
}
