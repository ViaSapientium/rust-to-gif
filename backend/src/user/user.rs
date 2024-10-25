use async_trait::async_trait;
use chrono::{DateTime, Utc};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use tokio_postgres::Error;
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub login: String,
  pub username: String,
  pub email: String,
  pub password: String, // Hashed password
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetToken {
  pub id: i32,
  pub user_id: i32,
  pub token: String,
  pub created_at: DateTime<Utc>,
  pub expires_at: DateTime<Utc>,
}

// Convert a database Row into a User instance
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

// Converting Row to PasswordResetToken
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

// Definition of the UserMethods trait for user-related methods
#[async_trait]
pub trait UserMethods {
  async fn find_by_login_or_email(
    client: &Client,
    login: &str,
    email: &str,
  ) -> Result<Option<User>, Error>;

  async fn add_user(
    client: &Client,
    username: &str,
    login: &str,
    email: &str,
    password: &str,
  ) -> Result<(), Error>;

  async fn all(client: &Client) -> Result<Vec<User>, Error>;

  async fn delete_user(client: &Client, login: &str) -> Result<(), Error>;

  async fn update_password(client: &Client, email: &str, new_password: &str) -> Result<(), Error>;
}

#[async_trait]
impl UserMethods for User {
  async fn find_by_login_or_email(
    client: &Client,
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
    client: &Client,
    username: &str,
    login: &str,
    email: &str,
    password: &str,
  ) -> Result<(), Error> {
    let stmt = client
      .prepare("INSERT INTO users (username, login, email, password) VALUES ($1, $2, $3, $4)")
      .await?;
    client
      .execute(&stmt, &[&username, &login, &email, &password])
      .await?;
    Ok(())
  }

  async fn all(client: &Client) -> Result<Vec<User>, Error> {
    let stmt = client
      .prepare("SELECT id, login, username, email, password FROM users")
      .await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows.into_iter().map(User::from).collect())
  }

  async fn delete_user(client: &Client, login: &str) -> Result<(), Error> {
    let stmt = client.prepare("DELETE FROM users WHERE login = $1").await?;
    client.execute(&stmt, &[&login]).await?;
    Ok(())
  }

  async fn update_password(client: &Client, email: &str, new_password: &str) -> Result<(), Error> {
    let stmt = client
      .prepare("UPDATE users SET password = $1 WHERE email = $2")
      .await?;
    client.execute(&stmt, &[&new_password, &email]).await?;
    Ok(())
  }
}

// Wrapper to work with DateTime<Utc> and serde
pub struct DateTimeWrapper(pub DateTime<Utc>);

impl<'a> tokio_postgres::types::FromSql<'a> for DateTimeWrapper {
  fn from_sql(
    _ty: &tokio_postgres::types::Type,
    raw: &[u8],
  ) -> Result<DateTimeWrapper, Box<dyn std::error::Error + Send + Sync>> {
    let s = std::str::from_utf8(raw)?;
    let dt = DateTime::parse_from_rfc3339(s)?;

    Ok(DateTimeWrapper(dt.with_timezone(&Utc)))
  }

  fn accepts(ty: &tokio_postgres::types::Type) -> bool {
    *ty == tokio_postgres::types::Type::TIMESTAMPTZ
  }
}
