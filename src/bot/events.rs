use serenity::{
    async_trait,
    model::{
        guild::Member,
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::CommandDataOptionValue, Interaction, InteractionResponseType,
            },
            GuildId, Ready,
        },
    },
    prelude::{Context, EventHandler},
};
use tracing::{debug, error, info, info_span, instrument, Instrument};
use uuid::Uuid;

use crate::{bot::utils::database::DatabasePool, nft::VerusNFTBuilder};

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

        // TODO let config_location = sqlx::query!(
        //     // get the latest config from the database
        // );

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
                        match create_nft(user_id, sequence as u64).await {
                            Ok(nft_builder) => {
                                // if the creation was ok, there should be a metadata JSON file.
                                // if let Err(e) = sqlx::query!(
                                //     "INSERT INTO user_register (discord_user_id, vrsc_address) VALUES ($1, $2)",
                                //     user_id as i64,
                                //     nft_builder.vrsc_address.to_string()
                                // )
                                // .execute(&pool)
                                // .await
                                // {
                                //     error!("Database write error: {:?}", e)
                                // }

                                match new_member.user.create_dm_channel(&ctx).await {
                                    Ok(dm) => {
                                        dm.say(&ctx, "Your NFT is ready!").await.unwrap();
                                        dm.say(
                                            &ctx,
                                            format!(
                                                "https://arweave.net/{}",
                                                nft_builder.uploaded_image_tx_hash.unwrap()
                                            ),
                                        )
                                        .await
                                        .unwrap();

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

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                "id" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let CommandDataOptionValue::User(user, _member) = options {
                        format!("{}'s id is {}", user.tag(), user.id)
                    } else {
                        "Please provide a valid user".to_string()
                    }
                }
                "attachmentinput" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected attachment option")
                        .resolved
                        .as_ref()
                        .expect("Expected attachment object");

                    if let CommandDataOptionValue::Attachment(attachment) = options {
                        format!(
                            "Attachment name: {}, attachment size: {}",
                            attachment.filename, attachment.size
                        )
                    } else {
                        "Please provide a valid attachment".to_string()
                    }
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            std::env::var("GUILD_ID")
                .expect("Expected GUILD_ID in env")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|cmd| {
                cmd.name("ping").description("ping me")
            })
            .create_application_command(|command| {
                command.name("id").description("Get a user id").create_option(|option| {
                    option
                        .name("id")
                        .description("The user to lookup")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
            })
            .create_application_command(|command| {
                command
                    .name("welcome")
                    .name_localized("de", "begrüßen")
                    .description("Welcome a user")
                    .description_localized("de", "Einen Nutzer begrüßen")
                    .create_option(|option| {
                        option
                            .name("user")
                            .name_localized("de", "nutzer")
                            .description("The user to welcome")
                            .description_localized("de", "Der zu begrüßende Nutzer")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_option(|option| {
                        option
                            .name("message")
                            .name_localized("de", "nachricht")
                            .description("The message to send")
                            .description_localized("de", "Die versendete Nachricht")
                            .kind(CommandOptionType::String)
                            .required(true)
                            .add_string_choice_localized(
                                "Welcome to our cool server! Ask me if you need help",
                                "pizza",
                                [("de", "Willkommen auf unserem coolen Server! Frag mich, falls du Hilfe brauchst")]
                            )
                            .add_string_choice_localized(
                                "Hey, do you want a coffee?",
                                "coffee",
                                [("de", "Hey, willst du einen Kaffee?")],
                            )
                            .add_string_choice_localized(
                                "Welcome to the club, you're now a good person. Well, I hope.",
                                "club",
                                [("de", "Willkommen im Club, du bist jetzt ein guter Mensch. Naja, hoffentlich.")],
                            )
                            .add_string_choice_localized(
                                "I hope that you brought a controller to play together!",
                                "game",
                                [("de", "Ich hoffe du hast einen Controller zum Spielen mitgebracht!")],
                            )
                    })
            })
            .create_application_command(|command| {
                command
                    .name("numberinput")
                    .description("Test command for number input")
                    .create_option(|option| {
                        option
                            .name("int")
                            .description("An integer from 5 to 10")
                            .kind(CommandOptionType::Integer)
                            .min_int_value(5)
                            .max_int_value(10)
                            .required(true)
                    })
                    .create_option(|option| {
                        option
                            .name("number")
                            .description("A float from -3.3 to 234.5")
                            .kind(CommandOptionType::Number)
                            .min_number_value(-3.3)
                            .max_number_value(234.5)
                            .required(true)
                    })
            })
            .create_application_command(|command| {
                command
                    .name("attachmentinput")
                    .description("Test command for attachment input")
                    .create_option(|option| {
                        option
                            .name("attachment")
                            .description("A file")
                            .kind(CommandOptionType::Attachment)
                            .required(true)
                    })
            })
        });

        commands.await;
        // debug!("{:?}", commands);
    }
}

async fn create_nft(user_id: u64, sequence: u64) -> Result<VerusNFTBuilder, ()> {
    // here is where we need to start generating an NFT.
    // TODO get config and directory locations from a separate config file.

    let series = String::from("geckotest");
    info!("creating {} nft #{} for {}", series, sequence, user_id);
    let nft_builder = crate::nft::VerusNFTBuilder::generate(user_id, sequence, series).await;

    Ok(nft_builder)

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

        // let mut join_handles = vec![];
    }
}
