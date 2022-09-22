use sqlx::postgres::PgPool;

use crate::configuration::DatabaseSettings;

// This function obtains a database connection to the postgresql database used for the bot.
pub async fn obtain_postgres_pool(
    db_settings: &DatabaseSettings,
) -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let pg_url = &db_settings.connection_string();

    let pool = PgPool::connect_lazy(pg_url)?;

    Ok(pool)
}
