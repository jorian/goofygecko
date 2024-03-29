use serenity::prelude::TypeMapKey;
use sqlx::PgPool;

pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

pub struct AppConfig;

impl TypeMapKey for AppConfig {
    type Value = crate::configuration::Settings;
}
