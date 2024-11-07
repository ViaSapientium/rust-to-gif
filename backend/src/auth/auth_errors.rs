use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthServiceError {
  #[error("Hashing error: {0}")]
  HashingError(String),
  #[error("Database error: {0}")]
  DatabaseError(#[from] tokio_postgres::Error),
  #[error("Invalid email")]
  InvalidEmail,
  #[error("Invalid password")]
  InvalidPassword,
  #[error("Invalid credentials")]
  InvalidCredentials,
  #[error("User not found")]
  UserNotFound,
  #[error("User already exists")]
  UserAlreadyExists,
  #[error("Unknown error")]
  Unknown,
  #[error("Invalid token")]
  InvalidToken,
}
