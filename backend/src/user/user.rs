use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use tokio_postgres::types::{FromSql, Type};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub login: String,
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetToken {
  pub id: i32,
  pub user_id: i32,
  pub token: String,
  pub created_at: DateTime<Utc>,
  pub expires_at: DateTime<Utc>,
}

// Conversion for database rows
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
