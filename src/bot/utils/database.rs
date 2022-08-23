use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPool;

pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}
