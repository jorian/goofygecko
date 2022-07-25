/*
Is concerned with:
- image generation
- metadata generation
- metadata retrieval
- rarity generator based on discord id
 */

// use std::path::{Path, PathBuf};

// use tracing::log::error;

pub(crate) mod art;
mod arweave;
mod config;
pub(crate) mod metadata;

/// an overarching struct that keeps track of all the details when generating a single NFT:
/// - art
/// - metadata creation and updates
/// - arweave details
/// - identity details
///

static CONFIG_LOCATION: &str = "assets/config.json";
static ASSETS_LOCATION: &str = "assets/";
static OUTPUT_LOCATION: &str = "generated/";

pub struct NFTBuilder {
    // every NFT has its own user_id:
    pub user_id: u64,
}

impl NFTBuilder {
    pub async fn generate(user_id: u64) {
        let mut nft_builder = Self { user_id };

        nft_builder.generate_metadata().await;
        nft_builder.generate_art().await;

        // nft_builder.arweave_upload();
    }

    async fn generate_metadata(&mut self) {
        metadata::generate(self.user_id, CONFIG_LOCATION.as_ref()).await
    }

    async fn generate_art(&self) {
        art::generate(
            self.user_id,
            ASSETS_LOCATION.as_ref(),
            OUTPUT_LOCATION.as_ref(),
        )
        .await
        .unwrap();
    }

    // async fn arweave_upload(&self) {
    //     let image_tx_id: &str = arweave::create_image_transaction(self.user_id);
    // }
}

pub struct NFTBuilderError {}
