use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;
use tokio_postgres_migration::Migration;
use deadpool_postgres::Runtime;

// Migrations up
const SCRIPTS_UP: [(&str, &str); 1] = [(
  "0001_create-users",
  include_str!("../migrations/0001_create-users_up.sql"),
)];

// Migrations down
const SCRIPTS_DOWN: [(&str, &str); 1] = [(
  "0001_drop-users",
  include_str!("../migrations/0001_create-users_down.sql"),
)];

fn create_config() -> Config {
  let mut cfg = Config::new();
  cfg.host = std::env::var("PG_HOST").ok();
  cfg.dbname = std::env::var("PG_DBNAME").ok();
  cfg.user = std::env::var("PG_USER").ok();
  cfg.password = std::env::var("PG_PASSWORD").ok();

  if cfg.host.is_none() || cfg.dbname.is_none() || cfg.user.is_none() || cfg.password.is_none() {
      panic!("Missing one or more PostgreSQL configuration variables!");
  }

  cfg
}

pub fn create_pool() -> Pool {
    let runtime = Runtime::Tokio1;
    create_config()
        .create_pool(Some(runtime), NoTls)
        .expect("couldn't create postgres pool")
}

async fn run_migration(pool: &Pool, scripts: &[(&str, &str)], direction: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut client = pool.get().await?;
  let migration = Migration::new("migrations".to_string());

  match direction {
      "up" => migration.up(&mut **client, scripts).await?,
      "down" => migration.down(&mut **client, scripts).await?,
      _ => panic!("Invalid migration direction"),
  }

  Ok(())
}

pub async fn migrate_up(pool: &Pool) {
  run_migration(pool, &SCRIPTS_UP, "up").await.expect("Failed to run migrations up");
}

pub async fn migrate_down(pool: &Pool) {
  run_migration(pool, &SCRIPTS_DOWN, "down").await.expect("Failed to run migrations down");
}

