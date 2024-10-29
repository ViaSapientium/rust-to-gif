use crate::user::user::User;
use crate::user::user_errors::UserError;
use async_trait::async_trait;
use deadpool_postgres::Client;

#[async_trait]
pub trait UserRepository {
  async fn add_user(
    client: &Client,
    username: &str,
    login: &str,
    email: &str,
    password: &str,
  ) -> Result<(), UserError>;

  async fn find_by_login_or_email(
    client: &Client,
    login: &str,
    email: &str,
  ) -> Result<Option<User>, UserError>;

  async fn delete_user(client: &Client, login: &str) -> Result<(), UserError>;

  async fn all(client: &Client) -> Result<Vec<User>, UserError>;
}

pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository for UserRepositoryImpl {
  async fn add_user(
    client: &Client,
    username: &str,
    login: &str,
    email: &str,
    password: &str,
  ) -> Result<(), UserError> {
    let stmt = client
      .prepare("INSERT INTO users (username, login, email, password) VALUES ($1, $2, $3, $4)")
      .await?;
    client
      .execute(&stmt, &[&username, &login, &email, &password])
      .await?;
    Ok(())
  }

  async fn find_by_login_or_email(
    client: &Client,
    login: &str,
    email: &str,
  ) -> Result<Option<User>, UserError> {
    let stmt = client
      .prepare(
        "SELECT id, login, username, email, password FROM users WHERE login = $1 OR email = $2",
      )
      .await?;
    let row = client.query_opt(&stmt, &[&login, &email]).await?;
    Ok(row.map(User::from))
  }

  async fn delete_user(client: &Client, login: &str) -> Result<(), UserError> {
    let stmt = client.prepare("DELETE FROM users WHERE login = $1").await?;
    client.execute(&stmt, &[&login]).await?;
    Ok(())
  }

  async fn all(client: &Client) -> Result<Vec<User>, UserError> {
    let stmt = client
      .prepare("SELECT id, login, username, email, password FROM users")
      .await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows.into_iter().map(User::from).collect())
  }
}
