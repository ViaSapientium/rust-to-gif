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
}
