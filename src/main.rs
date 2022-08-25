extern crate verusnftlib;

use std::{collections::HashSet, pin::Pin, sync::Arc};

use color_eyre::Report;
use futures::Future;
use load_dotenv::load_dotenv;
use serenity::{
    client::{ClientBuilder, Context},
    framework::standard::{macros::hook, DispatchError, StandardFramework},
    http::Http,
    model::{channel::Message, gateway::GatewayIntents},
};
use sqlx::PgPool;
use tracing::{debug, error, info, info_span, instrument, Instrument};
use tracing_subscriber::filter::EnvFilter;

use uuid::Uuid;
use verusnftlib::bot::{
    events, framework::*, utils::config::get_configuration, utils::database::DatabasePool,
};

#[tokio::main(worker_threads = 8)]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let configuration = get_configuration().unwrap();

    if configuration.enable_tracing {
        setup_logging().await?;

        info!("Tracer initialized");
    }

    debug!(
        "connection string: {}",
        configuration.database.connection_string()
    );

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
        // .before(before_hook) // returns a bool where the value of which will prevent running the event any further
        // .after(f)
        .group(&TEST_GROUP);

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

        sqlx::migrate!("./migrations")
            .run(&pg_pool)
            .await
            .expect("Failed to migrate the database");

        data.insert::<DatabasePool>(pg_pool);
    }

    info!("Starting client");

    // start listening on a separate thread and dispatch any events to their own threads.
    // (which means i don't have to care so much about threads!)
    if let Err(why) = client.start().await {
        error!(
            "An error occurred while running the discord bot client: {:?}",
            why
        );
    }

    Ok(())
}

#[instrument(skip(_ctx))]
pub fn before_hook<'fut>(
    _ctx: &'fut Context,
    msg: &'fut Message,
    cmd_name: &'fut str,
) -> Pin<Box<dyn Future<Output = bool> + Send + 'fut>> {
    Box::pin(async move {
        info!("Got command '{}' by user '{}'", cmd_name, msg.author.name);

        true
    })
}

async fn setup_logging() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "serenity=debug,verusnft=debug")
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
