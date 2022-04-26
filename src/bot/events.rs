use serenity::{
    async_trait,
    model::{guild::Member, id::GuildId, prelude::Ready},
    prelude::{Context, EventHandler},
};
use tracing::{debug, info};

#[derive(Debug)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, new_member: Member) {
        debug!(
            "A new member joined the discord with user_id {} and discriminant {}",
            new_member.user.id.0, new_member.user.discriminator
        );
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
