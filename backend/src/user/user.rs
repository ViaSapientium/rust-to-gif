use argon2::{
  password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use async_trait::async_trait;
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::error::Error as StdError;
use tokio_postgres::types::{FromSql, Type};
use tokio_postgres::Error;
use tokio_postgres::GenericClient;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
  pub id: i32,
  pub login: String,
  pub username: String,
  pub email: String,
  pub password: String,
}

pub struct PasswordResetToken {
  pub id: i32,
  pub user_id: i32,
  pub token: String,
  pub created_at: DateTime<Utc>,
  pub expires_at: DateTime<Utc>,
}

pub struct DateTimeWrapper(pub DateTime<Utc>);

impl<'a> FromSql<'a> for DateTimeWrapper {
  fn from_sql(_ty: &Type, raw: &[u8]) -> Result<DateTimeWrapper, Box<dyn StdError + Sync + Send>> {
    let s = std::str::from_utf8(raw)?;
    let dt = DateTime::parse_from_rfc3339(s)?;
    Ok(DateTimeWrapper(dt.with_timezone(&Utc)))
  }

  fn accepts(ty: &Type) -> bool {
    *ty == Type::TIMESTAMPTZ
  }
}

impl From<Row> for User {
  fn from(row: Row) -> Self {
    User {
      id: row.get("id"),
      login: row.get("login"),
      username: row.get("username"),
      email: row.get("email"),
      password: row.get("password"),
    }
  }
}

impl From<Row> for PasswordResetToken {
  fn from(row: Row) -> Self {
    PasswordResetToken {
      id: row.get("id"),
      user_id: row.get("user_id"),
      token: row.get("token"),
      created_at: row.get::<_, DateTimeWrapper>("created_at").0,
      expires_at: row.get::<_, DateTimeWrapper>("expires_at").0,
    }
  }
}

#[async_trait]
pub trait UserMethods {
  async fn find_by_login_or_email(
    client: &(impl GenericClient + Sync),
    login: &str,
    email: &str,
  ) -> Result<Option<User>, tokio_postgres::Error>;

  async fn add_user(
    client: &(impl GenericClient + Sync),
    username: &str,
    email: &str,
    password: &str,
  ) -> Result<(), tokio_postgres::Error>;
}

#[async_trait]
impl UserMethods for User {
  async fn find_by_login_or_email(
    client: &(impl GenericClient + Sync),
    login: &str,
    email: &str,
  ) -> Result<Option<User>, tokio_postgres::Error> {
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
  ) -> Result<(), tokio_postgres::Error> {
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

impl PasswordResetToken {
  pub async fn create_token<C: GenericClient>(client: &C, user_id: i32) -> Result<String, Error> {
    let token = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + ChronoDuration::hours(1)).timestamp();
    let stmt = client
      .prepare("INSERT INTO password_reset_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)")
      .await?;
    client
      .execute(&stmt, &[&user_id, &token, &(expires_at as i32)])
      .await?;
    Ok(token)
  }

  pub async fn check_token<C: GenericClient>(
    client: &C,
    token: &str,
  ) -> Result<Option<PasswordResetToken>, Error> {
    let stmt = client.prepare("SELECT id, user_id, token, created_at, expires_at FROM password_reset_tokens WHERE token = $1 AND expires_at > EXTRACT(EPOCH FROM NOW())").await?;
    let row = client.query_opt(&stmt, &[&token]).await?;
    Ok(row.map(PasswordResetToken::from))
  }

  pub async fn delete_by_token<C: GenericClient>(client: &C, token: &str) -> Result<(), Error> {
    let stmt = client
      .prepare("DELETE FROM password_reset_tokens WHERE token = $1")
      .await?;
    client.execute(&stmt, &[&token]).await?;
    Ok(())
  }
}
