use serenity::{
    async_trait,
    model::{guild::Member, id::GuildId},
    prelude::{Context, EventHandler},
};
use tracing::debug;

#[derive(Debug)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, new_member: Member) {
        debug!("A new member joined the discord!");
        debug!("{:?}", new_member.user.id);
    }
}
