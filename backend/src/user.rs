use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub username: String,
    pub email: String,
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
}
