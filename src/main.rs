extern crate verusnftlib;

use std::sync::Arc;

use color_eyre::Report;
use load_dotenv::load_dotenv;
use tracing::{debug, error, info};
use tracing_subscriber::filter::EnvFilter;

use serenity::{
    client::{Client, Context},
    framework::standard::{
        macros::{command, group, hook},
        CommandResult, DispatchError, StandardFramework,
    },
    model::{channel::Message, gateway::GatewayIntents},
};

use verusnftlib::bot::{events, utils, utils::database::DatabasePool};

#[group]
#[commands(ping)]
struct General;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    load_dotenv!();

    setup_logging().await?;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .on_dispatch_error(on_dispatch_error)
        .group(&GENERAL_GROUP);

    let token = env!("DISCORD_TOKEN");
    let handler = Arc::new(events::Handler {});

    let mut intents = GatewayIntents::all();
    intents.remove(GatewayIntents::DIRECT_MESSAGE_TYPING);
    intents.remove(GatewayIntents::GUILD_MESSAGE_TYPING);

    let mut client = Client::builder(token, intents)
        .event_handler_arc(handler.clone())
        .framework(framework)
        .await
        .expect("Error creating serenity client");

    {
        // in a block to close the write borrow
        let mut data = client.data.write().await;

        let pg_pool = utils::database::obtain_postgres_pool().await?;
        data.insert::<DatabasePool>(pg_pool);
    }

    debug!("starting client");

    if let Err(why) = client.start().await {
        println!(
            "An error occurred while running the discord bot client: {:?}",
            why
        );
    }

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    info!("Message received: {}", &msg.content);
    info!("{}", msg.author.id);

    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

async fn setup_logging() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "serenity=info,verusnft=debug")
    }
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

#[hook]
pub async fn on_dispatch_error(
    ctx: &Context,
    msg: &Message,
    error: DispatchError,
    _command_name: &str,
) {
    match error {
        DispatchError::OnlyForDM => {
            info!("Only in DM");
            let _ = msg
                .reply(ctx, "This can only be done in DM with this bot")
                .await;
        }
        _ => {
            error!("Unhandled dispatch error: {:?}", error);
            eprintln!("An unhandled dispatch error has occurred:");
            eprintln!("{:?}", error);
        }
    }
}
