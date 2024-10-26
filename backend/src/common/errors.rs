use thiserror::Error;
use tokio_postgres::Error as PostgresError;

#[derive(Error, Debug)]
pub enum AuthServiceError {
  #[error("Hashing error")]
  HashingError(String),

  #[error("Database error")]
  DatabaseError(#[from] PostgresError),
}

#[derive(Error, Debug)]
pub enum UserServiceError {
  #[error("Password hashing error")]
  HashingError(String), // Change Argon2Error en String pour éviter les conflits

  #[error("Database error")]
  DatabaseError(#[from] PostgresError),
}
