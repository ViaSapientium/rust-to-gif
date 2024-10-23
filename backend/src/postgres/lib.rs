use deadpool_postgres::{Pool, PoolError};
use std::io;

pub async fn test_connection(pool: &Pool) -> Result<(), io::Error> {
  let client = pool
    .get()
    .await
    .map_err(|e: PoolError| io::Error::new(io::ErrorKind::Other, format!("Pool error: {}", e)))?;
  client
    .batch_execute("SELECT 1")
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Postgres error: {}", e)))?;
  println!("Connexion PostgresSQL réussie");
  Ok(())
}
