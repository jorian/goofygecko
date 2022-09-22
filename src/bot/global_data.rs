use serenity::prelude::TypeMapKey;
use sqlx::PgPool;

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

pub struct AppConfig;

impl TypeMapKey for AppConfig {
    type Value = crate::configuration::Settings;
}
