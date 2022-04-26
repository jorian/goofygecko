use load_dotenv::load_dotenv;
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPool;

pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

// This function obtains a database connection to the postgresql database used for the bot.
pub async fn obtain_postgres_pool() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    // Obtain the postgresql url.
    load_dotenv!();
    let pg_url = env!("DATABASE_URL2");

    // Connect to the database with the information provided on the configuration.
    // and return a pool of connections
    let pool = PgPool::connect_lazy(&pg_url)?;
    // let pool = PgPoolOptions::new()
    //     .max_connections(20)
    //     .connect_lazy(&pg_url)?;

    // return the pool
    Ok(pool)
}
