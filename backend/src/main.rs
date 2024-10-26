use crate::postgres::config::create_pool;
use actix_web::{web, App, HttpServer, Responder};
use deadpool_postgres::Pool;
use dotenvy::dotenv;

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

  let pool = create_pool();

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
      .route("/users", web::get().to(user::user_controller::all_users))
      .route("/users", web::post().to(user::user_controller::create_user))
      .route(
        "/users/{login}",
        web::delete().to(user::user_controller::delete_user),
      )
      .route(
        "/users/password",
        web::put().to(user::user_controller::update_password),
      )
  })
  .bind("127.0.0.1:8081")?
  .run()
  .await
}
