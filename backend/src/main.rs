use actix_web::{web, App, HttpServer, Responder};
use deadpool_postgres::Pool;
use dotenvy::dotenv;
use std::env;

mod postgres;
mod user;

// Route simple pour tester le serveur
async fn index() -> impl Responder {
  "Rust GIF API is running!"
}

// Route pour tester la connexion à PostgreSQL
async fn db_check(pool: web::Data<Pool>) -> impl Responder {
  match postgres::lib::test_connection(&pool).await {
    Ok(_) => "PostgreSQL connection successful!",
    Err(_) => "PostgreSQL connection failed!",
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok(); // Charge les variables d'environnement à partir de .env

  // Création du pool de connexion PostgreSQL
  let pool = postgres::config::create_pool();

  let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone())) // Ajout du pool dans le contexte
      .route("/", web::get().to(index)) // Route principale
      .route("/db-check", web::get().to(db_check)) // Route pour vérifier la connexion à PostgreSQL
  })
  .bind(("127.0.0.1", port.parse().unwrap()))?
  .run()
  .await
}
