use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use deadpool_postgres::Pool;
use rand::{distributions::Alphanumeric, Rng};
mod postgres;
mod user;

// Manage the address on which the application listens
fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

// List users
#[get("/users")]
async fn list_users(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    
    match user::User::all(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::debug!("unable to fetch users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch users");
        }
    }
}

// Search for a user by login or email
#[get("/users/find")]
async fn find_user(pool: web::Data<Pool>, query: web::Query<(String, String)>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };

    let (login, email) = query.into_inner();
    match user::User::find_by_login_or_email(&**client, &login, &email).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json("User not found"),
        Err(err) => {
            log::debug!("unable to find user: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to find user");
        }
    }
}

// Resets the database
#[get("/reset-database")]
async fn reset_database(pool: web::Data<Pool>) -> HttpResponse {
    postgres::migrate_down(&pool).await;
    HttpResponse::Ok().json("Database reset successfully")
}

#[post("/forgot-password")]
async fn forgot_password(pool: web::Data<Pool>, form: web::Form<(String,)>) -> HttpResponse {
    let email = &form.0;
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().json("Erreur serveur")
    };

    // Find user by email
    let user = match user::User::find_by_email(&**client, email).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().json("Utilisateur non trouvé"),
        Err(_) => return HttpResponse::InternalServerError().json("Erreur serveur"),
    };

    // Generate a reset token
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    // Create the token in the database
    match user::PasswordResetToken::create(&**client, user.id, &token).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().json("Erreur lors de la création du token"),
    }

    // Send email (display link in response)
    let reset_link = format!("https://mon-site/reset-password?token={}", token);
    HttpResponse::Ok().json(reset_link)
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Create the database connection
    let pg_pool = postgres::create_pool();

    // Manage migrations based on environment variables
    if std::env::var("ROLLBACK_MIGRATION").is_ok() {
        postgres::migrate_down(&pg_pool).await;
    } else {
        postgres::migrate_up(&pg_pool).await;
    }

    let address = address();

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .service(list_users)
            .service(find_user)
            .service(reset_database)
    })
    .bind(&address)?
    .run()
    .await
}
