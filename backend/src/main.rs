use actix_web::{web, App, HttpServer, Responder};
use deadpool_postgres::{Config, Pool, Runtime};
use dotenvy::dotenv;
use std::env;
// use ffmpeg_next::format::context::input::Input;
// use video::ffmpeg::extract_images;

mod auth;
mod common;
mod postgres;
mod user;
// mod video;

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
/* async fn extract_video() -> impl Responder {
  let video_path = "/src/assets/video.mp4";
  let output_dir = "/src/video/output";

  match extract_images(video_path, output_dir) {
    Ok(_) => HttpResponse::Ok().body("Images extracted successfully!"),
    Err(e) => HttpResponse::In,
  }
}
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let mut cfg = Config::new();
  cfg.dbname = Some(env::var("PG_DBNAME").expect("PG_DBNAME must be set"));
  cfg.user = Some(env::var("PG_USER").expect("PG_USER must be set"));
  cfg.password = Some(env::var("PG_PASSWORD").expect("PG_PASSWORD must be set"));
  cfg.host = Some(env::var("PG_HOST").expect("PG_HOST must be set"));

  let pool = cfg
    .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
    .unwrap();

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone())) // Contexte
      .route("/", web::get().to(index))
      .route("/db-check", web::get().to(db_check))
      .route("/login", web::post().to(auth::auth_controller::login))
      .route("/register", web::post().to(auth::auth_controller::register))
      .route(
        "/validate",
        web::get().to(auth::auth_service::AuthService::validate_token),
      )
  })
  .bind("127.0.0.1:8081")?
  .run()
  .await
}
