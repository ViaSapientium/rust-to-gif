use actix_web::{web, App, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;

mod postgres;
mod user;

// Route simple pour tester le serveur
async fn index() -> impl Responder {
  "Rust GIF API is running!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok(); // Charge les variables d'environnement à partir de .env

  let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
  HttpServer::new(|| {
    App::new().route("/", web::get().to(index)) // Route principale
  })
  .bind(("127.0.0.1", port.parse().unwrap()))?
  .run()
  .await
}
