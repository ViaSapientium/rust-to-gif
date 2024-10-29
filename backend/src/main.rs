use crate::postgres::lib::test_connection;
use actix_web::{web, App, HttpServer, Responder};
use auth::auth_routes::configure_auth_routes;
use config::create_pool;
use deadpool_postgres::Pool;
use dotenvy::dotenv;
use user::user_routes::configure_user_routes;
// use video::ffmpeg::extract_images;
// use ffmpeg_next::format::context::input::Input;

mod auth;
mod common;
mod config;
mod postgres;
mod user;
// mod video;

// Route simple pour tester le serveur
async fn index() -> impl Responder {
  "Rust GIF API is running!"
}

// Route pour tester la connexion à PostgreSQL
async fn db_check(pool: web::Data<Pool>) -> impl Responder {
  match test_connection(&pool).await {
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

  let pool = create_pool();

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone())) // Contexte
      .route("/", web::get().to(index))
      .route("/db_check", web::get().to(db_check))
      .configure(|cfg| configure_auth_routes(cfg))
      .configure(|cfg| configure_user_routes(cfg))
    // .route("/extract_video", web::get().to(extract_video))
  })
  .bind("127.0.0.1:8081")?
  .run()
  .await
}
