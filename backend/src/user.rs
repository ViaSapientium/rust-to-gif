use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};
use chrono::{Duration, Utc};

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub username: String,
    pub email: String,
}

pub struct PasswordResetToken {
  pub id: i32,
  pub user_id: i32,
  pub token: String,
  pub created_at: chrono::NaiveDateTime,
  pub expires_at: chrono::NaiveDateTime,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            login: row.get(1),
            username: row.get(2),
            email: row.get(3),
        }
    }
}

impl From<Row> for PasswordResetToken {
  fn from(row: Row) -> Self {
    Self {
      id: row.get(0),
      user_id: row.get(1),
      token: row.get(2),
      created_at: row.get(3),
      expires_at: row.get(4),
    }
  }
}

impl User {
    // Method to recover all users
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<User>, Error> {
        let stmt = client.prepare("SELECT id, login, username, email FROM users").await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(User::from).collect())
    }

    // Method to search for a user by login or email
    pub async fn find_by_login_or_email<C: GenericClient>(client: &C, login: &str, email: &str) -> Result<Option<User>, Error> {
        let stmt = client.prepare("SELECT id, login, username, email FROM users WHERE login = $1 OR email = $2").await?;
        let row = client.query_opt(&stmt, &[&login, &email]).await?;

        Ok(row.map(User::from))
    }

    // Add a new user
    pub async fn add_user<C: GenericClient>(client: &C, login: &str, username: &str, email: &str) -> Result<(), Error> {
        let stmt = client.prepare("INSERT INTO users (login, username, email) VALUES ($1, $2, $3)").await?;
        client.execute(&stmt, &[&login, &username, &email]).await?;
        Ok(())
    }

    // Delete a user by login
    pub async fn delete_by_login<C: GenericClient>(client: &C, login: &str) -> Result<(), Error> {
        let stmt = client.prepare("DELETE FROM users WHERE login = $1").await?;
        client.execute(&stmt, &[&login]).await?;
        Ok(())
    }

    // Reset token storage
    pub async fn store_reset_token<C: GenericClient>(client: &C, email: &str, token: &str, expires_at: u64) -> Result<(), Error> {
      let stmt = client.prepare("INSERT INTO password_reset_tokens (email, token, expires_at) VALUES ($1, $2, $3)").await?;
      client.execute(&stmt, &[&email, &token, &(expires_at as i64)]).await?;
      Ok(())
    }

    // Verify the reset token
    pub async fn verify_reset_token<C: GenericClient>(client: &C, token: &str) -> Result<Option<User>, Error> {
      let stmt = client.prepare("SELECT users.id, users.login, users.username, users.email FROM users JOIN password_reset_tokens ON users.email = password_reset_tokens.email WHERE password_reset_tokens.token = $1 AND password_reset_tokens.expires_at > EXTRACT(EPOCH FROM NOW())").await?;
      let row = client.query_opt(&stmt, &[&token]).await?;
      Ok(row.map(User::from))
    }

    // Update the password
    pub async fn update_password<C: GenericClient>(client: &C, email: &str, new_password: &str) -> Result<(), Error> {
      let stmt = client.prepare("UPDATE users SET password = $1 WHERE email = $2").await?;
      client.execute(&stmt, &[&new_password, &email]).await?;
      Ok(())
    }

    // Invalidate the reset token
    pub async fn invalidate_reset_token<C: GenericClient>(client: &C, token: &str) -> Result<(), Error> {
      let stmt = client.prepare("DELETE FROM password_reset_tokens WHERE token = $1").await?;
      client.execute(&stmt, &[&token]).await?;
      Ok(())
    }
}

impl PasswordResetToken {
  // Method to recover all users
  pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<PasswordResetToken>, Error> {
      let stmt = client.prepare("SELECT id, user_id, token, created_at, expires_at FROM password_reset_tokens").await?;
      let rows = client.query(&stmt, &[]).await?;

      Ok(rows.into_iter().map(PasswordResetToken::from).collect())
  }

  // Delete a user by login
  pub async fn delete_by_token<C: GenericClient>(client: &C, token: &str) -> Result<(), Error> {
      let stmt = client.prepare("DELETE FROM password_reset_tokens WHERE token = $1").await?;
      client.execute(&stmt, &[&token]).await?;
      Ok(())
  }

  // Create a new token for the user
  pub async fn create_token<C: GenericClient>(client: &C, user_id: i32) -> Result<String, Error> {
    let token = uuid::Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::hours(1)).timestamp();
    let stmt = client.prepare("INSERT INTO password_reset_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)").await?;
    client.execute(&stmt, &[&user_id, &token, &(expires_at as i64)]).await?;
    Ok(token)
  }

  // Check if a token exists and has not expired
  pub async fn check_token<C: GenericClient>(client: &C, token: &str) -> Result<Option<PasswordResetToken>, Error> {
    let stmt = client.prepare("SELECT id, user_id, token, created_at, expires_at FROM password_reset_tokens WHERE token = $1 AND expires_at > EXTRACT(EPOCH FROM NOW())").await?;
    let row = client.query_opt(&stmt, &[&token]).await?;
    Ok(row.map(PasswordResetToken::from))
  }
}
