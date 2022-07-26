use serenity::{
    async_trait,
    model::{guild::Member, prelude::Ready},
    prelude::{Context, EventHandler},
};
use tracing::{debug, error, info};

use crate::bot::utils::database::DatabasePool;

#[derive(Debug)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
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

            // this process can take a while, so we spawn it in a tokio thread
            // tokio::spawn is parallelism. It hooks into the runtime executor as a new future.
            tokio::spawn(async move {
                // path is the location of the NFT image locally.
                // TODO that path should be a Arweave tx
                match create_nft(user_id, 0).await {
                    Ok(_path) => {
                        // if the creation was ok, there should be a metadata JSON file.
                        // if let Err(e) = sqlx::query!(
                        //     "INSERT INTO user_register (discord_user_id) VALUES ($1)",
                        //     user_id as i64
                        // )
                        // .execute(&pool)
                        // .await
                        // {
                        //     error!("Database write error: {:?}", e)
                        // }

                        match new_member.user.create_dm_channel(&ctx).await {
                            Ok(dm) => {
                                dm.say(&ctx, "Your NFT is ready!").await.unwrap();

                                // TODO required:
                                // - image of the NFT (link to arweave)
                                // arweave::get_image() for the gecko that belongs to user_id
                                // - name of the NFT (get previously stored database item (SELECT name FROM nft_names WHERE user_id = 'new_member.user.id'))
                                // - tips to show NFT in verusnft discord
                            }
                            Err(e) => {
                                error!("Sending DM to new user error: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Something went wrong while creating the NFT: {:?}", e)
                        // TODO something that notifies me
                    }
                }
            });
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

async fn create_nft(user_id: u64, i: u64) -> Result<(), ()> {
    // here is where we need to start generating an NFT.
    // TODO get config and directory locations from a separate config file.

    crate::nft::NFTBuilder::generate(user_id).await;
    println!("{}", i);

    Ok(())

    // let config_path_buf = Path::new("./assets/config.json");
    // if config_path_buf.exists() {
    //     crate::nft::metadata::generate(user_id, &config_path_buf);
    // } else {
    //     error!("config file does not exist: {}", config_path_buf.display());
    // }
}

#[cfg(test)]
mod tests {
    use super::create_nft;
    use rand::{prelude::SliceRandom, Rng};

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn create_metadata() {
        let mut rng = rand::thread_rng();
        let user_id: u64 = rng.gen_range(0..123456789);

        let mut join_handles = vec![];

        for i in 0..10 {
            join_handles.push(tokio::spawn(async move {
                create_nft(user_id + i, i).await.unwrap();
            }))
        }

        join_handles.shuffle(&mut rng);

        let _ = futures::future::try_join_all(join_handles).await;
        // tokio::spawn(async move {
        //     join!(join_handles);
        // });
    }
}
