use serenity::{
    async_trait,
    model::{
        id::GuildId, guild::Member, application::{interaction::Interaction}, prelude::Ready,
    },
    prelude::{Context, EventHandler},
};
use tracing::{debug, error, info, info_span, instrument, Instrument};
use uuid::Uuid;

use crate::{bot::utils::database::{DatabasePool, GuildId as GId, SequenceStart}, nft::VerusNFTBuilder};

#[derive(Debug)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip(ctx), fields(
        request_id = %Uuid::new_v4()
    ))]
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

        if let Some(row) = data {
            debug!("a member entered that previously entered; ignore")
        } else {
            debug!("this is a first-time new member, adding to user_register");
            // get a sequential number to number the new gecko:
            let next_gecko_number = sqlx::query!("SELECT nextval('goofygeckoserial')")
                .fetch_one(&pool)
                .await
                .unwrap();

            debug!(
                "the next Gecko number is: {:?}",
                next_gecko_number.nextval.unwrap()
            );

            // this process can take a while, so we spawn it in a tokio thread
            // tokio::spawn is parallelism. It hooks into the runtime executor as a new future.
            tokio::spawn(
                async move {
                    // path is the location of the NFT image locally.
                    // TODO that path should be a Arweave tx
                    if let Some(sequence) = next_gecko_number.nextval {
                        let data_read = ctx.data.read().await;
                        let sequence_start = data_read.get::<SequenceStart>().unwrap().clone();

                        let sequence = sequence + sequence_start;
                        match create_nft(user_id, sequence as u64).await {
                            Ok(nft_builder) => {
                                // if the creation was ok, there should be a metadata JSON file.
                                if let Err(e) = sqlx::query!(
                                    "INSERT INTO user_register (discord_user_id, vrsc_address) VALUES ($1, $2)",
                                    user_id as i64,
                                    nft_builder.vrsc_address.to_string()
                                )
                                .execute(&pool)
                                .await
                                {
                                    error!("Database write error: {:?}", e)
                                }

                                match new_member.user.create_dm_channel(&ctx).await {
                                    Ok(dm) => {
                                        dm.say(&ctx, "Your NFT is ready!").await.unwrap();
                                        dm.say(
                                            &ctx,
                                            format!(
                                                "https://arweave.net/{}",
                                                &nft_builder.uploaded_image_tx_hash.as_ref().unwrap()
                                            ),
                                        )
                                        .await
                                        .unwrap();
                                        
                                        dm.say(&ctx, format!("See the metadata of this file: <https://v2.viewblock.io/arweave/tx/{}>", &nft_builder.uploaded_metadata_tx_hash.as_ref().unwrap()))
                                        .await
                                        .unwrap();
                                        
                                        dm.say(&ctx, format!("See the image transaction: <https://v2.viewblock.io/arweave/tx/{}>", &nft_builder.uploaded_image_tx_hash.as_ref().unwrap()))
                                        .await
                                        .unwrap();

                                        let data_read = ctx.data.read().await;
                                        let guild_id = data_read.get::<GId>().unwrap().clone();

                                        let channels = ctx.http.get_channels(guild_id).await.unwrap();
                                        let channel = channels.iter().find(|c| c.name == "general").expect("could not find 'general' channel");

                                        channel.send_message(&ctx.http, |m| {
                                            m.embed(|e| {
                                                e.title(format!("Introducing testgecko #{}", nft_builder.sequence))
                                                .description(format!("**Rarity:** {}\n**Price:** {} VRSC", 23, 12))
                                                .field("Transaction", format!("[view](https://v2.viewblock.io/arweave/tx/{})", nft_builder.uploaded_image_tx_hash.as_ref().unwrap()), true)
                                                .field("Metadata", format!("[view](https://v2.viewblock.io/arweave/tx/{})", nft_builder.uploaded_metadata_tx_hash.as_ref().unwrap()), true)
                                                .image(format!(
                                                    "https://arweave.net/{}",
                                                    &nft_builder.uploaded_image_tx_hash.as_ref().unwrap()
                                            ))})
                                        }).await.unwrap();

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
                    }
                }
                .instrument(info_span!("new_nft")),
            );
        }
    }

    #[instrument(skip(ctx, interaction), fields(
        request_id = %Uuid::new_v4()
    ))]
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("received command interaction: {:?}", command);

            match command.data.name.as_str() {
                "list" => {
                    
                    let data_read = ctx.data.read().await;
                    let pg_pool = data_read.get::<DatabasePool>().clone().unwrap();
                

                    let current_user = ctx.http.get_current_user().await.unwrap().id.0;

                    let query = sqlx::query!("SELECT vrsc_address FROM user_register WHERE discord_user_id = $1", current_user as i64)
                        .fetch_optional(pg_pool)
                        .await
                        .unwrap();

                    debug!("{:#?}", query);
                },
                _ => {}
                
            };

        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_id = {
            let data_read = ctx.data.read().await;
            GuildId(*data_read.get::<GId>().unwrap())
        };

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|cmd| {
                cmd.name("list").description("List all my NFTs")
            });

            commands

        }).await;
    }

}

async fn create_nft(user_id: u64, sequence: u64) -> Result<VerusNFTBuilder, ()> {
    // here is where we need to start generating an NFT.
    // TODO get config and directory locations from a separate config file.

    let series = String::from("geckotest");
    info!("creating {} nft #{} for {}", series, sequence, user_id);
    let nft_builder = crate::nft::VerusNFTBuilder::generate(user_id, sequence, series).await;

    Ok(nft_builder)
}

