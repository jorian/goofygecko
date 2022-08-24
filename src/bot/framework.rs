use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
};
use tracing::{debug, error, info, info_span, instrument};
use uuid::Uuid;

use crate::bot::utils::database::DatabasePool;

#[group("Test")]
#[commands(ping, create_nft)]
pub struct Test;

#[command]
// #[instrument(skip(ctx, msg))]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    info!("Message received: {}", &msg.content);
    info!("{}", msg.author.id);

    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
#[aliases(createnft)]
async fn create_nft(ctx: &Context, msg: &Message) -> CommandResult {
    // let span = info_span!();;

    inner_create_nft(ctx, msg).await;

    Ok(())
}

// #[instrument(skip(ctx), fields(
//     request_id = %Uuid::new_v4(),
//     member_id = %msg.author.id.0
// ))]
async fn inner_create_nft(ctx: &Context, msg: &Message) {
    let user_id = msg.author.id.0;
    debug!(
        "A new member joined the discord with user_id {} and discriminant {}",
        user_id, msg.author.discriminator
    );

    let pool = {
        let data_read = &ctx.data.read().await;
        data_read.get::<DatabasePool>().unwrap().clone()
    };

    // get a sequential number to number the new gecko:
    // let sequence = 14;
    let next_gecko_number = sqlx::query!("SELECT nextval('goofygeckoserial')")
        .fetch_one(&pool)
        .await
        .unwrap();

    let sequence = next_gecko_number.nextval.unwrap();

    debug!("creating NFT #{}", sequence);

    // this process can take a while, so we spawn it in a tokio thread
    // tokio::spawn is parallelism. It hooks into the runtime executor as a new future.
    // tokio::spawn(async move {
    // path is the location of the NFT image locally.
    // TODO that path should be a Arweave tx
    // if let Some(sequence) = next_gecko_number.nextval {
    match super::create_nft(user_id, sequence as u64).await {
        Ok(nft_builder) => {
            msg.reply(
                &ctx,
                format!(
                    "https://arweave.net/{}",
                    nft_builder.uploaded_image_tx_hash.unwrap()
                ),
            )
            .await
            .unwrap();

            if let Some(identity) = nft_builder.identity.as_ref() {
                msg.reply(
                    &ctx,
                    format!(
                        "txid of identity registration: https://testex.verus.io/tx/{}",
                        identity.registration_txid
                    ),
                )
                .await
                .unwrap();
            }
        }
        Err(e) => {
            error!("Something went wrong while creating the NFT: {:?}", e)
            // TODO something that notifies me
        }
    }
}
