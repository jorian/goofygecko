use tracing::info;

use crate::nft::VerusNFTBuilder;

pub mod commands;
pub mod events;
pub mod framework;
mod global_data;
mod logging;
pub mod utils;

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
