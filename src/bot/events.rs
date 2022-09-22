use crate::{
    bot::utils::{
        database::{DatabasePool, GuildId as GId, SequenceStart},
        embeds,
    },
    nft::VerusNFT,
};
use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction,
        guild::Member,
        id::GuildId,
        prelude::{
            command::CommandOptionType, interaction::application_command::CommandDataOptionValue,
            Ready,
        },
    },
    prelude::{Context, EventHandler},
};
use sqlx::{query, Pool, Postgres};
use tracing::{debug, error, info, instrument};
use uuid::Uuid;
use vrsc_rpc::{Auth, Client, RpcApi};

#[derive(Debug)]
pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip(ctx, interaction), fields(
        request_id = %Uuid::new_v4()
    ))]
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("received command interaction: {:?}", command);

            match command.data.name.as_str() {
                "gecko" => {
                    let option = command
                        .data
                        .options
                        .get(0)
                        .expect("user option")
                        .resolved
                        .as_ref()
                        .expect("an integer indicating a gecko");

                    if let CommandDataOptionValue::Integer(n) = option {
                        debug!("got number {} to look up", n);
                        // get identity
                        let client = Client::chain("vrsctest", Auth::ConfigFile, None)
                            .expect("A verus daemon client");
                        let identity_res =
                            client.get_identity(&format!("{}.geckotest@", *n as u64));

                        debug!("{:?}", identity_res);

                        if let Ok(identity) = identity_res {
                            let cm = identity.identity.contentmap;
                            let hex_tx = cm
                                .get("9a55eaaad7bacc9f37a449e315ff32fedc07b126")
                                .expect("a vdxf key that indicates the location of the metadata");

                            let hex_decoded = hex::decode(hex_tx).expect("hex decode");

                            debug!("retrieved from contentmap: {:?}", hex_decoded);
                            let encoded_tx_hash_str = base64_url::encode(&hex_decoded);
                            // base64_url::encode(base64_tx).expect("a base64 url");
                            debug!("encoded_tx_hash: {:?}", &encoded_tx_hash_str);
                            // let decoded_tx_hash_str = std::str::from_utf8(decoded_tx_hash_vec.as_ref())
                            //     .expect("a valid utf8 string");

                            let metadata =
                                crate::nft::arweave::get_metadata_json(&encoded_tx_hash_str).await;

                            let pool = {
                                let data_read = &ctx.data.read().await;
                                data_read.get::<DatabasePool>().unwrap().clone()
                            };

                            let guild_id = {
                                let data_read = &ctx.data.read().await;
                                *data_read.get::<GId>().unwrap()
                            };

                            let mut owner = String::from("_not in Discord_");

                            for address in identity.identity.primaryaddresses {
                                let query = query!("SELECT discord_user_id FROM user_register WHERE vrsc_address = $1", address.to_string());
                                if let Ok(result) = query.fetch_optional(&pool).await {
                                    if let Some(record) = result {
                                        let discord_user_id = record.discord_user_id;
                                        if let Ok(member) = ctx
                                            .http
                                            .get_member(guild_id, discord_user_id as u64)
                                            .await
                                        {
                                            owner = member.user.tag();
                                        }
                                    }
                                }
                            }
                            debug!("owner: {}", owner);

                            command
                                .create_interaction_response(&ctx.http, |response| {
                                    response.interaction_response_data(|data| {
                                        data.embed(|e| {
                                            e.title(metadata.name)
                                                .description(format!(
                                                    "**Rarity:** {}\n",
                                                    metadata.rarity
                                                ))
                                                .field("Owner", owner, true)
                                                .field(
                                                    "Metadata",
                                                    format!(
                                                    "[view](https://v2.viewblock.io/arweave/tx/{})",
                                                    encoded_tx_hash_str
                                                ),
                                                    true,
                                                )
                                                .image(format!(
                                                    "https://arweave.net/{}",
                                                    &metadata.image
                                                ))
                                        });
                                        // data.content(format!("https://arweave.net/{}", metadata.image));
                                        data.ephemeral(false)
                                    })
                                })
                                .await
                                .expect("a response to a /gecko interaction");
                        }
                    } else {
                        error!("no number was entered")
                    }
                }
                "list" => {
                    let data_read = ctx.data.read().await;
                    let pg_pool = data_read.get::<DatabasePool>().clone().unwrap();

                    let current_user = command.user.id.0;
                    debug!("current user: {}", current_user);

                    let query = sqlx::query!(
                        "SELECT vrsc_address FROM user_register WHERE discord_user_id = $1",
                        current_user as i64
                    )
                    .fetch_optional(pg_pool)
                    .await
                    .unwrap();

                    debug!("{:#?}", query);

                    let address = query
                        .expect("some record for this user")
                        .vrsc_address
                        .expect("an address for this user");

                    // we now have the connection between the discord user and the primary address for this user. Let's use the new RPC to find out which identities are controlled with this primary address:
                    let client = Client::chain("vrsctest", Auth::ConfigFile, None)
                        .expect("A verus daemon client");
                    let identities_with_address = client
                        .get_identities_with_address(&address, None, None, None)
                        .expect("An array of identities");

                    debug!("{:?}", identities_with_address);

                    for identity in identities_with_address {
                        let cm = identity.contentmap;
                        let hex_tx = cm
                            .get("9a55eaaad7bacc9f37a449e315ff32fedc07b126")
                            .expect("a vdxf key that indicates the location of the metadata");

                        let hex_decoded = hex::decode(hex_tx).expect("hex decode");

                        debug!("retrieved from contentmap: {:?}", hex_decoded);
                        let encoded_tx_hash_str = base64_url::encode(&hex_decoded);
                        // base64_url::encode(base64_tx).expect("a base64 url");
                        debug!("encoded_tx_hash: {:?}", &encoded_tx_hash_str);
                        // let decoded_tx_hash_str = std::str::from_utf8(decoded_tx_hash_vec.as_ref())
                        //     .expect("a valid utf8 string");

                        let metadata =
                            crate::nft::arweave::get_metadata_json(&encoded_tx_hash_str).await;

                        command
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|data| {
                                    data.embed(|e| {
                                        e.title(format!("Introducing {}", metadata.name))
                                            .description(format!(
                                                "**Rarity:** {}\n",
                                                metadata.rarity
                                            ))
                                            .field(
                                                "Transaction",
                                                format!(
                                                    "[view](https://v2.viewblock.io/arweave/tx/{})",
                                                    metadata.image
                                                ),
                                                true,
                                            )
                                            .field(
                                                "Metadata",
                                                format!(
                                                    "[view](https://v2.viewblock.io/arweave/tx/{})",
                                                    encoded_tx_hash_str
                                                ),
                                                true,
                                            )
                                            .image(format!(
                                                "https://arweave.net/{}",
                                                &metadata.image
                                            ))
                                    });
                                    // data.content(format!("https://arweave.net/{}", metadata.image));
                                    data.ephemeral(true)
                                })
                            })
                            .await
                            .expect("a response to a /list interaction");
                    }
                }
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
            commands
                .create_application_command(|cmd| cmd.name("list").description("List all my NFTs"))
                .create_application_command(|cmd| {
                    cmd.name("gecko")
                        .description("testest")
                        .create_option(|option| {
                            option
                                .name("number")
                                .description("The Goofy Gecko to look up")
                                .kind(CommandOptionType::Integer)
                                .required(true)
                        })
                })
        });

        let result = commands.await;
        debug!("Registered commands: {:?}", result);
        if let Err(error) = result {
            panic!("Commands were not registered successfully:\n{:#?}", error);
        }

        info!("Bot is ready!");
    }

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
            info!("a member entered that previously entered; ignore")
        } else {
            info!("this is a first-time new member");

            process_new_member(new_member, pool, ctx).await;
        }
    }
}

async fn create_nft(user_id: u64, sequence: u64) -> Result<VerusNFT, ()> {
    // here is where we need to start generating an NFT.
    // TODO get config and directory locations from a separate config file.

    let series = String::from("geckotest");
    info!("creating {} nft #{} for {}", series, sequence, user_id);
    let nft_builder = crate::nft::VerusNFT::generate(user_id, sequence, series).await;

    Ok(nft_builder)
}

async fn process_new_member(new_member: Member, pool: Pool<Postgres>, ctx: Context) {
    let next_gecko_number = sqlx::query!("SELECT nextval('goofygeckoserial')")
        .fetch_one(&pool)
        .await
        .unwrap();

    debug!(
        "the next Gecko number is: {:?}",
        next_gecko_number.nextval.unwrap()
    );

    // path is the location of the NFT image locally.
    // TODO that path should be a Arweave tx
    if let Some(sequence) = next_gecko_number.nextval {
        let data_read = ctx.data.read().await;
        let sequence_start = data_read.get::<SequenceStart>().unwrap().clone();

        let sequence = sequence + sequence_start;
        match create_nft(new_member.user.id.0, sequence as u64).await {
            Ok(verus_nft) => {
                // if the creation was ok, there should be a metadata JSON file.
                if let Err(e) = sqlx::query!(
                    "INSERT INTO user_register (discord_user_id, vrsc_address) VALUES ($1, $2)",
                    new_member.user.id.0 as i64,
                    verus_nft.vrsc_address.to_string()
                )
                .execute(&pool)
                .await
                {
                    error!("Database write error: {:?}", e)
                }

                match new_member.user.create_dm_channel(&ctx).await {
                    Ok(dm) => {
                        let data_read = ctx.data.read().await;
                        let guild_id = data_read.get::<GId>().unwrap().clone();

                        let channels = ctx.http.get_channels(guild_id).await.unwrap();
                        let channel = channels
                            .iter()
                            .find(|c| c.name == "general")
                            .expect("could not find 'general' channel");

                        channel
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| embeds::from_verusnft(e, verus_nft))
                            })
                            .await
                            .unwrap();
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
