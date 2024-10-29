use super::user::User;
use argon2::{
  password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use async_trait::async_trait;
use tokio_postgres::{Client, Error, GenericClient};

#[async_trait]
pub trait UserMethods {
  async fn find_by_login_or_email(
    client: &(impl GenericClient + Sync),
    login: &str,
    email: &str,
  ) -> Result<Option<User>, Error>;

  async fn add_user(
    client: &(impl GenericClient + Sync),
    username: &str,
    email: &str,
    password: &str,
  ) -> Result<(), Error>;

  async fn find_by_login(client: &Client, login: &str) -> Result<Option<User>, Error> {
    let stmt = client
      .prepare("SELECT id, login, username, email, password FROM users WHERE login = $1")
      .await?;
    let row = client.query_opt(&stmt, &[&login]).await?;
    Ok(row.map(User::from))
  }
}

#[async_trait]
impl UserMethods for User {
  async fn find_by_login_or_email(
    client: &(impl GenericClient + Sync),
    login: &str,
    email: &str,
  ) -> Result<Option<User>, Error> {
    let stmt = client
      .prepare(
        "SELECT id, login, username, email, password FROM users WHERE login = $1 OR email = $2",
      )
      .await?;
    let row = client.query_opt(&stmt, &[&login, &email]).await?;
    Ok(row.map(User::from))
  }

  async fn add_user(
    client: &(impl GenericClient + Sync),
    username: &str,
    email: &str,
    password: &str,
  ) -> Result<(), Error> {
    let stmt = client
      .prepare("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
      .await?;
    client
      .execute(&stmt, &[&username, &email, &password])
      .await?;
    Ok(())
  }
}

impl User {
  pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<User>, Error> {
    let stmt = client
      .prepare("SELECT id, login, username, email, password FROM users")
      .await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows.into_iter().map(User::from).collect())
  }

  pub async fn delete_user<C: GenericClient>(client: &C, login: &str) -> Result<(), Error> {
    let stmt = client.prepare("DELETE FROM users WHERE login = $1").await?;
    client.execute(&stmt, &[&login]).await?;
    Ok(())
  }

  pub async fn update_password<C: GenericClient>(
    client: &C,
    email: &str,
    new_password: &str,
  ) -> Result<(), Error> {
    let stmt = client
      .prepare("UPDATE users SET password = $1 WHERE email = $2")
      .await?;
    client.execute(&stmt, &[&new_password, &email]).await?;
    Ok(())
  }

  pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
  }

  pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(
      argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok(),
    )
  }
}
