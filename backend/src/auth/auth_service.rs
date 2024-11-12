use crate::auth::auth_dto::{LoginResponse, RegisterRequest, UpdatePasswordRequest};
use crate::auth::auth_errors::AuthServiceError;
use crate::auth::auth_jwt::generate_jwt;
use crate::user::user::User;
use crate::user::user_methods::UserMethods;
use actix_web::HttpResponse;
use argon2::{
  self,
  password_hash::{PasswordHash, SaltString},
  Argon2, PasswordHasher, PasswordVerifier,
};
use serde_json::json;
use tokio_postgres::GenericClient;

pub struct AuthService;

impl AuthService {
  pub async fn login(
    email: &str,
    password: &str,
    client: &(impl GenericClient + Sync),
  ) -> Result<LoginResponse, AuthServiceError> {
    if let Some(user) = User::find_by_login_or_email(client, email, email).await? {
      if !Self::verify_password(&user.password, password) {
        return Err(AuthServiceError::InvalidCredentials);
      }

      let token = generate_jwt(&user.email);
      Ok(LoginResponse {
        success: true,
        token: Some(token),
        message: Some("Login successful".to_string()),
      })
    } else {
      Err(AuthServiceError::UserNotFound)
    }
  }

  pub async fn register(
    body: RegisterRequest,
    client: &(impl GenericClient + Sync),
  ) -> Result<LoginResponse, AuthServiceError> {
    let email = &body.email;

    if let Some(_) = User::find_by_login_or_email(client, email, email).await? {
      return Err(AuthServiceError::UserAlreadyExists);
    }

    let hashed_password = Self::hash_password(&body.password)
      .map_err(|_| AuthServiceError::HashingError("Password hashing failed".to_string()))?;

    User::add_user(client, &body.username, email, &hashed_password).await?;
    let token = generate_jwt(email);

    Ok(LoginResponse {
      success: true,
      token: Some(token),
      message: Some("Registration successful".to_string()),
    })
  }

  pub async fn update_password(
    body: UpdatePasswordRequest,
    client: &(impl GenericClient + Sync),
  ) -> Result<(), AuthServiceError> {
    let email = &body.email;

    if let Some(user) = User::find_by_login_or_email(client, email, email).await? {
      let hashed_password = Self::hash_password(&body.new_password)
        .map_err(|_| AuthServiceError::HashingError("Password hashing failed".to_string()))?;

      User::update_password(client, &user.id, &hashed_password).await?;
      Ok(())
    } else {
      Err(AuthServiceError::UserNotFound)
    }
  }

  pub async fn validate_token(token: &str) -> Result<String, AuthServiceError> {
    match crate::auth::auth_jwt::validate_jwt(token) {
      Ok(email) => Ok(email),
      Err(_) => Err(AuthServiceError::InvalidToken),
    }
  }

  fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).ok();
    let argon2 = Argon2::default();
    parsed_hash
      .map(|hash| argon2.verify_password(password.as_bytes(), &hash).is_ok())
      .unwrap_or(false)
  }

  fn hash_password(password: &str) -> Result<String, AuthServiceError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    argon2
      .hash_password(password.as_bytes(), &salt)
      .map(|ph| ph.to_string())
      .map_err(|_| AuthServiceError::HashingError("Password hashing failed".to_string()))
  }
}
