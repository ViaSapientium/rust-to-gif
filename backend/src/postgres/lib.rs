use deadpool_postgres::Pool;
use std::error::Error;

// Fonction pour tester la connexion au pool PostgresSQL
pub async fn test_connection(pool: &Pool) -> Result<(), Box<dyn Error>> {
    let client = pool.get().await?;
    client.query("SELECT 1", &[]).await?;
    println!("Connexion PostgresSQL réussie");
    Ok(())
}
