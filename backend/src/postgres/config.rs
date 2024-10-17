use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use std::env;

pub fn create_pool() -> Pool {
  let mut cfg = Config::new();

  cfg.dbname = Some(env::var("PG_DBNAME").unwrap());
  cfg.user = Some(env::var("PG_USER").unwrap());
  cfg.password = Some(env::var("PG_PASSWORD").unwrap());
  cfg.host = Some(env::var("PG_HOST").unwrap());

  cfg.manager = Some(ManagerConfig {
    recycling_method: RecyclingMethod::Fast,
  });

  // Ajout du runtime pour le nouvel argument
  cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
