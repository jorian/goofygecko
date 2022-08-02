/*
Is concerned with:
- image generation
- metadata generation
- metadata retrieval
- rarity generator based on discord id
 */

// use std::path::{Path, PathBuf};

// use tracing::log::error;

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use serde_json::{json, Value};
use serenity::prelude::{Mutex, RwLock};
use tracing::{debug, error};

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

pub struct VerusNFT {}

#[derive(Debug)]
pub struct VerusNFTBuilder {
    // every NFT has its own user_id:
    pub user_id: u64,
    pub sequence: u64,
    pub generated_image_path: Option<PathBuf>,
    pub generated_metadata_path: Option<PathBuf>,
    pub uploaded_image_tx_hash: Option<String>,
    pub verus_commitment_tx_id: Option<String>, // TODO txid
    pub verus_registration_tx_id: Option<String>,
}

impl VerusNFTBuilder {
    pub async fn generate(user_id: u64, sequence: u64) -> Self {
        let mut nft_builder = Self {
            user_id,
            sequence,
            generated_metadata_path: None,
            generated_image_path: None,
            uploaded_image_tx_hash: None,
            verus_commitment_tx_id: None,
            verus_registration_tx_id: None,
        };

        nft_builder.generate_metadata().await;
        nft_builder.generate_art().await;

        debug!("art created");

        nft_builder.arweave_image_upload().await;

        nft_builder.update_metadata().await;

        debug!("{:?}", nft_builder.uploaded_image_tx_hash);

        Self {
            user_id,
            sequence,
            generated_image_path: None,
            generated_metadata_path: None,
            uploaded_image_tx_hash: nft_builder.uploaded_image_tx_hash.clone(),
            verus_commitment_tx_id: None,
            verus_registration_tx_id: None,
        }
    }

    /// Generates the metadata for the user that just entered and stores it locally.
    async fn generate_metadata(&mut self) {
        metadata::generate(self.user_id, self.sequence, CONFIG_LOCATION.as_ref()).await;

        // TODO this is really ugly, need one source of truth
        // ideally metadata::generate returns the location of the generated metadata, which I can update here:
        let path = PathBuf::from_str(&format!("{}{}.json", OUTPUT_LOCATION, self.user_id))
            .expect("parsing metadata path failed");

        self.generated_metadata_path = Some(path);
    }

    async fn generate_art(&mut self) {
        match art::generate(
            self.user_id,
            ASSETS_LOCATION.as_ref(),
            OUTPUT_LOCATION.as_ref(),
        )
        .await
        {
            Ok(path) => {
                self.generated_image_path = Some(path);
            }
            Err(e) => {
                error!("error while generating art file: {:?}", e)
            }
        }
    }

    async fn arweave_image_upload(&mut self) {
        if let Some(path) = self.generated_image_path.clone() {
            let mut arweave_tx =
                arweave::ArweaveTransaction::new(Path::new(".ardrivewallet.json")).await;

            debug!("arweave instance created");

            match arweave_tx.upload(&path, String::from("image/png")).await {
                Ok(tx_hash) => {
                    self.uploaded_image_tx_hash = Some(tx_hash);
                }
                Err(e) => {
                    error!("could not upload image: {:?}", e);
                }
            }
        } else {
            error!("no generated_image_path was found for: {}", self.user_id);
        }
    }

    async fn update_metadata(&self) {
        // self.metadata
        // self.image_hash
        // put image_hash in metadata.

        // read metadata file
        // update `image` key with actual location on Arweave
        // save metadata file

        if let Some(image_hash) = self.uploaded_image_tx_hash.clone() {
            if let Some(path) = self.generated_metadata_path.clone() {
                let metadata_file = fs::read_to_string(&path).unwrap();
                let mut metadata: Value = serde_json::from_str(&metadata_file).unwrap();

                if let Some(image) = metadata.get_mut("image") {
                    *image = json!(image_hash);
                }

                let mut file = File::create(&path)
                    .expect(&format!("Could not create file at path {}", path.display()));

                write!(file, "{}", metadata).expect(&format!(
                    "Could not write to file at path {}",
                    path.display()
                ));
            } else {
                error!("no generated metadata file was found");
            }
        } else {
            error!("no image hash to insert in metadata file");
        }
    }

    async fn arweave_metadata_upload(&mut self) {
        // upload
    }

    async fn create_verus_id_commitment(&self) {
        // need to wait for it to be confirmed.
    }

    async fn create_verus_id_registration(&self) {
        // add metadata hashes of metadata to content map
    }
}

pub struct NFTBuilderError {}
