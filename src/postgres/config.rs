use deadpool_postgres::{Config, Pool};
use std::env;
use tokio_postgres::NoTls;

pub fn create_pool() -> Pool {
  let mut cfg = Config::new();

  // Chargement des valeurs depuis les variables d'environnement
  cfg.host = Some(env::var("PG_HOST").expect("PG_HOST must be set"));
  cfg.user = Some(env::var("PG_USER").expect("PG_USER must be set"));
  cfg.password = Some(env::var("PG_PASSWORD").expect("PG_PASSWORD must be set"));
  cfg.dbname = Some(env::var("PG_DBNAME").expect("PG_DBNAME must be set"));

  cfg.create_pool(None, NoTls).unwrap()
}
