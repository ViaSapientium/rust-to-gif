use crate::common::errors::UserServiceError;
use crate::user::user::User;
use crate::user::user::UserMethods;
use argon2::{
  password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use deadpool_postgres::Client;

pub struct UserService;

impl From<argon2::password_hash::Error> for UserServiceError {
  fn from(err: argon2::password_hash::Error) -> Self {
    UserServiceError::HashingError(err.to_string())
  }
}

impl UserService {
  pub async fn add_user(
    client: &Client,
    username: &str,
    login: &str,
    email: &str,
    password: &str,
  ) -> Result<(), UserServiceError> {
    let hashed_password = Self::hash_password(password)?;
    User::add_user(client, username, login, email, &hashed_password).await?;
    Ok(())
  }

  pub async fn update_password(
    client: &Client,
    email: &str,
    new_password: &str,
  ) -> Result<(), UserServiceError> {
    let hashed_password = Self::hash_password(new_password)?;
    User::update_password(client, email, &hashed_password).await?;
    Ok(())
  }

  pub async fn delete_user(client: &Client, login: &str) -> Result<(), UserServiceError> {
    User::delete_user(client, login).await?;
    Ok(())
  }

  pub async fn all_users(client: &Client) -> Result<Vec<User>, UserServiceError> {
    let users = User::all(client).await?;
    Ok(users)
  }

  fn hash_password(password: &str) -> Result<String, UserServiceError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
  }

  pub fn verify_password(hash: &str, password: &str) -> Result<bool, UserServiceError> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(
      argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok(),
    )
  }
}
