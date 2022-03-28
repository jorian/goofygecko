use color_eyre::Report;
use load_dotenv::load_dotenv;
use tracing::{debug, error, info};
use tracing_subscriber::filter::EnvFilter;
// use serenity::async_trait;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group, hook},
        CommandResult, DispatchError, StandardFramework,
    },
    model::channel::Message,
    model::gateway::Ready,
};

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    load_dotenv!();

    setup_logging().await?;


    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .on_dispatch_error(on_dispatch_error)
        .group(&GENERAL_GROUP);

    let token = env!("DISCORD_TOKEN");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating discord bot client");

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
pub async fn on_dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::OnlyForDM => {
            info!("Only in DM");
            let _ = msg
                .reply(ctx, format!("This can only be done in DM with this bot"))
                .await;
        }
        _ => {
            error!("Unhandled dispatch error: {:?}", error);
            eprintln!("An unhandled dispatch error has occurred:");
            eprintln!("{:?}", error);
        }
    }
}
