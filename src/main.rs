extern crate verusnftlib;

use std::{collections::HashSet, sync::Arc};

use color_eyre::Report;
use load_dotenv::load_dotenv;
use sqlx::PgPool;
use tracing::{error, info, instrument};
use tracing_subscriber::filter::EnvFilter;

use serenity::{
    client::{ClientBuilder, Context},
    framework::standard::{
        macros::{command, group, hook},
        CommandResult, DispatchError, StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::GatewayIntents},
};

use verusnftlib::bot::{events, utils::config::get_configuration, utils::database::DatabasePool};

#[group]
#[commands(ping)]
struct General;

#[tokio::main(worker_threads = 8)]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let mut file = File::open("config.toml")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    // // gets the data from the config.toml file
    // let configuration = toml::from_str::<Settings>(&contents).unwrap();
    let configuration = get_configuration().unwrap();

    if configuration.enable_tracing {
        setup_logging().await?;

        info!("Tracer initialized");
    }

    load_dotenv!();

    let bot_token = env!("DISCORD_TOKEN");
    let http = Http::new(&bot_token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            let current_user = http.get_current_user().await?;

            (owners, current_user.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("").on_mention(Some(bot_id)).owners(owners))
        .on_dispatch_error(on_dispatch_error)
        // .before(f) // returns a bool where the value of which will prevent running the event any further
        // .after(f)
        .group(&GENERAL_GROUP);

    let handler = Arc::new(events::Handler {});

    let mut intents = GatewayIntents::all();
    intents.remove(GatewayIntents::DIRECT_MESSAGE_TYPING);
    intents.remove(GatewayIntents::GUILD_MESSAGE_TYPING);

    let mut client = ClientBuilder::new(&bot_token, intents)
        .event_handler_arc(handler.clone())
        .framework(framework)
        .await?;

    {
        // in a block to close the write borrow
        let mut data = client.data.write().await;

        let pg_options = configuration.database.with_db();
        let pg_pool = PgPool::connect_lazy_with(pg_options);
        data.insert::<DatabasePool>(pg_pool);
    }

    info!("Starting client");

    // start listening on a separate thread and dispatch any events to their own threads.
    // (which means i don't have to care so much about threads!)
    if let Err(why) = client.start_autosharded().await {
        error!(
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

    info!("Subscriber initialized");

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
            if let Err(e) = msg
                .reply(ctx, "This can only be done in DM with this bot")
                .await
            {
                error!("something went wrong while sending a reply in DM: {:?}", e);
            }
        }
        _ => {
            error!("Unhandled dispatch error: {:?}", error);
        }
    }
}
