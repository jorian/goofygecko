use std::path::Path;
use std::path::PathBuf;

use serenity::{
    async_trait,
    model::{guild::Member, id::GuildId, prelude::Ready},
    prelude::{Context, EventHandler},
};
use tracing::{debug, error, info};

use crate::bot::utils::database::DatabasePool;

#[derive(Debug)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        // fn cb_nft_ready();

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

        // TODO let config_location = sqlx::query!(
        //     // get the latest config from the database
        // );

        if let Some(row) = data {
            debug!("a member entered that previously entered; ignore")
        } else {
            debug!("this is a first-time new member, adding to user_register");

            tokio::spawn(async move {
                // path is the location of the NFT locally.
                if let Ok(_path) = create_nft(user_id) {
                    // if the creation was ok, there should be a metadata JSON file.
                    if let Err(e) = sqlx::query!(
                        "INSERT INTO user_register (discord_user_id) VALUES ($1)",
                        user_id as i64
                    )
                    .execute(&pool)
                    .await
                    {
                        error!("Database write error: {:?}", e)
                    }

                    match new_member.user.create_dm_channel(&ctx).await {
                        Ok(dm) => {
                            dm.say(&ctx, "You NFT is ready!").await.unwrap();

                            // TODO required:
                            // - image of the NFT (link to arweave)
                            // - name of the NFT (get previously stored database item (SELECT name FROM nft_names WHERE user_id = 'new_member.user.id'))
                            // - tips to show NFT in verusnft discord
                        }
                        Err(e) => {
                            error!("Sending DM to new user error: {:?}", e);
                        }
                    }
                }
            })
            .await
            .unwrap();

            // TODO add callback when nft creation is done
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn create_nft(user_id: u64) -> Result<PathBuf, ()> {
    // here is where we need to start generating an NFT.
    // TODO get config and directory locations from a separate config file.

    let config_path_buf = Path::new("./assets/config.json");
    if config_path_buf.exists() {
        crate::nft::metadata::generate(user_id, &config_path_buf);
    }

    crate::nft::art::generate(user_id, Path::new("./assets"), Path::new("./generated"))
}
