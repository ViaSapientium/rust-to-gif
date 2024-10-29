use crate::user::user::User;
use crate::user::user_errors::UserError;
use crate::user::user_repository::{UserRepository, UserRepositoryImpl};
use argon2::{
  self,
  password_hash::{PasswordHash, PasswordVerifier, SaltString},
  Argon2,
};
use deadpool_postgres::Client;
use rand::rngs::OsRng;
use rand::{Rng, RngCore};

pub struct UserService;

impl UserService {
  pub async fn add_user(
    client: &Client,
    username: &str,
    login: &str,
    email: &str,
    password: Option<&str>,
  ) -> Result<(), UserError> {
    let password = password.unwrap_or_else(|| generate_password(24));
    let hashed_password = Self::hash_password(Some(&password))?;
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

  pub fn generate_password(len: usize) -> String {
    let mut rng = OsRng;
    (0..len)
      .map(|_| rng.gen_range(b'A'..=b'Z') as char)
      .collect()
  }
}
