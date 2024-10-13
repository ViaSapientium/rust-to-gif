use actix_web::{get, web, App, HttpResponse, HttpServer};
use deadpool_postgres::Pool;

mod postgres;
mod user;

// Liste les utilisateurs
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

// Recherche un utilisateur par login ou email
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

// Gère l'adresse sur laquelle l'application écoute
fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

// Réinitialise la base de données
#[get("/reset-database")]
async fn reset_database(pool: web::Data<Pool>) -> HttpResponse {
    postgres::migrate_down(&pool).await;
    HttpResponse::Ok().json("Database reset successfully")
}

// Fonction principale
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Crée la connexion à la base de données
    let pg_pool = postgres::create_pool();

    // Gère les migrations en fonction des variables d'environnement
    if std::env::var("ROLLBACK_MIGRATION").is_ok() {
        postgres::migrate_down(&pg_pool).await;
    } else {
        postgres::migrate_up(&pg_pool).await;
    }

    let address = address();

    // Démarre le serveur HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .service(list_users)  // Endpoint pour lister les utilisateurs
            .service(find_user)    // Endpoint pour trouver un utilisateur
            .service(reset_database)  // Endpoint pour réinitialiser la base de données
    })
    .bind(&address)?
    .run()
    .await
}
