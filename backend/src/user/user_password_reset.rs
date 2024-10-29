use super::user::PasswordResetToken;
use chrono::{Duration as ChronoDuration, Utc};
use tokio_postgres::{Error, GenericClient};
use uuid::Uuid;

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
