use actix_web::{web, App, HttpServer, Responder};
use deadpool_postgres::Pool;
use dotenvy::dotenv;
use std::env;
use video::ffmpeg::extract_images;

mod postgres;
mod user;
mod video;

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

// Route pour extraire des images depuis une vidéo
async fn extract_video() -> impl Responder {
  let video_path = "/src/assets/video.mp4";
  let output_dir = "/src/video/output";

  match extract_images(video_path, output_dir) {
    Ok(_) => HttpResponse::Ok().body("Images extracted successfully!"),
    Err(e) => HttpResponse::In,
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  // Création du pool de connexion PostgreSQL
  let pool = postgres::config::create_pool(); // Cette ligne n'a pas besoin d'un match

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
