use crate::user::user::User;
use crate::user::user_errors::UserError;
use crate::user::user_repository::{UserRepository, UserRepositoryImpl};
use crate::utils::password_generator::generate_password;
use crate::utils::password_validator::validate_password;
use argon2::{
  self, password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use deadpool_postgres::Client;
use rand::rngs::OsRng;

pub struct UserService;

impl UserService {
  pub async fn add_user(
    client: &Client,
    username: &str,
    login: &str,
    email: &str,
    password: Option<&str>,
  ) -> Result<(), UserError> {
    let password = password
      .map(|p| p.to_string())
      .unwrap_or_else(generate_password);

    if !validate_password(&password) {
      return Err(UserError::InvalidPassword);
    }

    let hashed_password = Self::hash_password(&password)?;
    UserRepositoryImpl::add_user(client, username, login, email, &hashed_password).await
  }

  pub async fn find_by_login_or_email(
    client: &Client,
    login: &str,
    email: &str,
  ) -> Result<Option<User>, UserError> {
    UserRepositoryImpl::find_by_login_or_email(client, login, email).await
  }

  pub async fn delete_user(client: &Client, login: &str) -> Result<(), UserError> {
    UserRepositoryImpl::delete_user(client, login).await
  }

  pub async fn all_users(client: &Client) -> Result<Vec<User>, UserError> {
    UserRepositoryImpl::all(client).await
  }

  pub async fn update_password(
    client: &Client,
    email: &str,
    new_password: &str,
  ) -> Result<(), UserError> {
    let hashed_password = Self::hash_password(&new_password)?;
    UserRepositoryImpl::update_password(client, email, &hashed_password).await
  }

  pub fn verify_password(hash: &str, password: &str) -> Result<bool, UserError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| UserError::HashingError)?;
    let argon2 = Argon2::default();
    Ok(
      argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok(),
    )
  }

  fn hash_password(password: &str) -> Result<String, UserError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
      .hash_password(password.as_bytes(), &salt)
      .map_err(|_| UserError::HashingError)?;
    Ok(password_hash.to_string())
  }
}
