use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
  #[error("User not found")]
  NotFound,

  #[error("User already exists")]
  AlreadyExists,

  #[error("Database error")]
  DatabaseError(#[from] tokio_postgres::Error),

  #[error("Password hashing error")]
  HashingError,

  #[error("Invalid password")]
  InvalidPassword,

  #[error("Unknown error")]
  Unknown,

  #[error("Invalid email")]
  InvalidEmail,

  #[error("Invalid login")]
  InvalidLogin,

  #[error("Invalid username")]
  InvalidUsername,
}
