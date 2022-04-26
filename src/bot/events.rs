use serenity::{
    async_trait,
    model::{guild::Member, id::GuildId, prelude::Ready},
    prelude::{Context, EventHandler},
};
use tracing::{debug, info};

use crate::bot::utils::database::DatabasePool;

#[derive(Debug)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        let user_id = new_member.user.id.0;
        debug!(
            "A new member joined the discord with user_id {} and discriminant {}",
            user_id, new_member.user.discriminator
        );

        let pool = {
            let data_read = &ctx.data.read().await;
            data_read.get::<DatabasePool>().unwrap().clone()
        };

        let data = sqlx::query!(
            "SELECT discord_user_id FROM user_register WHERE discord_user_id = $1",
            user_id as i64
        )
        .fetch_optional(&pool)
        .await
        .unwrap();

        if let Some(row) = data {
            debug!("a member entered that previously entered; ignore")
        } else {
            debug!("this is a first-time new member, adding to user_register");

            // TODO here is where we need to start generating an NFT.
            crate::nft::metadata::generate(user_id);

            sqlx::query!(
                "INSERT INTO user_register (discord_user_id) VALUES ($1)",
                user_id as i64
            )
            .execute(&pool)
            .await
            .unwrap(); // TODO handle error
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
