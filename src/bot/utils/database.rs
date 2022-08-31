use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPool;

use crate::configuration::DatabaseSettings;

pub struct SequenceStart;
impl TypeMapKey for SequenceStart {
    type Value = i64;
}
pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

pub struct GuildId;

impl TypeMapKey for GuildId {
    type Value = u64;
}

// This function obtains a database connection to the postgresql database used for the bot.
pub async fn obtain_postgres_pool(
    db_settings: &DatabaseSettings,
) -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let pg_url = &db_settings.connection_string();

    let pool = PgPool::connect_lazy(pg_url)?;

    Ok(pool)
}
