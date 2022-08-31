use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPool;

use crate::configuration::DatabaseSettings;

pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

// This function obtains a database connection to the postgresql database used for the bot.
pub async fn obtain_postgres_pool(
    db_settings: &DatabaseSettings,
) -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    // Obtain the postgresql url.
    // load_dotenv!();
    // let pg_url = env!("DATABASE_URL2");

    let pg_url = &db_settings.connection_string();
    // Connect to the database with the information provided on the configuration.
    // and return a pool of connections
    let pool = PgPool::connect_lazy(pg_url)?;
    // let pool = PgPoolOptions::new()
    //     .max_connections(20)
    //     .connect_lazy(&pg_url)?;

    // return the pool
    Ok(pool)
}
