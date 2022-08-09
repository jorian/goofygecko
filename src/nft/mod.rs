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
};

use serde_json::{json, Value};
// use serenity::prelude::{Mutex, RwLock};
use tracing::{debug, error, info};
use vrsc_rpc::{json::vrsc::Address, Auth, Client, RpcApi};

use super::identity::Identity;

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
    pub vrsc_address: Address,
    pub sequence: u64,
    pub edition: String,
    pub generated_image_path: Option<PathBuf>,
    pub generated_metadata_path: Option<PathBuf>,
    pub uploaded_image_tx_hash: Option<String>,
    pub uploaded_metadata_tx_hash: Option<String>,
    pub identity: Option<Identity>,
}

impl VerusNFTBuilder {
    pub async fn generate(user_id: u64, sequence: u64, series: String) -> Self {
        let client = Client::chain("vrsctest", Auth::ConfigFile, None);
        let address = match client {
            Ok(client) => client.get_new_address().unwrap(),
            Err(e) => {
                error!("an error happened while getting a new address: {:?}", e);
                panic!("{:?}", e);
            }
        };

        let mut nft_builder = Self {
            user_id,
            vrsc_address: address,
            sequence,
            edition: series,
            generated_metadata_path: None,
            generated_image_path: None,
            uploaded_image_tx_hash: Some(String::from(
                "8BvUWr1sdZDLINUkWmwqdttO22cQdhoP2MlyVpTG2d8",
            )),
            uploaded_metadata_tx_hash: Some(String::from(
                "gHaQrrqU34a1oPpjWjr9u9ruUFzaT1-lwdjTBHnu9Z4",
            )),
            identity: None,
        };

        nft_builder.generate_metadata().await;
        nft_builder.generate_art().await;
        // nft_builder.arweave_image_upload().await;
        nft_builder.update_metadata().await;
        // nft_builder.arweave_metadata_upload().await;
        nft_builder.create_identity().await;

        nft_builder
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
            self.uploaded_image_tx_hash = Self::arweave_upload(
                &path,
                vec![
                    ("Content-Type", "image/png"),
                    ("identity", &format!("{}.{}@", self.sequence, &self.edition)),
                ],
            )
            .await
            .ok()
        } else {
            error!("no generated_image_path was found for: {}", self.user_id);
        }
    }

    async fn arweave_metadata_upload(&mut self) {
        if let Some(path) = self.generated_metadata_path.clone() {
            self.uploaded_metadata_tx_hash = Self::arweave_upload(
                &path,
                vec![
                    ("Content-Type", "application/json"),
                    ("vdxfid", &format!("{}.{}@", self.sequence, &self.edition)), //TODO set actual vdxfid
                ],
            )
            .await
            .ok()
        } else {
            error!("no generated_metadata_path was found for: {}", self.user_id);
        }
    }

    async fn arweave_upload(path: &Path, tags: Vec<(&str, &str)>) -> Result<String, ()> {
        let mut arweave_tx =
            arweave::ArweaveTransaction::new(Path::new(".ardrivewallet.json")).await;

        match arweave_tx.upload(&path, tags).await {
            Ok(tx_hash) => Ok(tx_hash),
            Err(e) => {
                error!("could not upload image: {:?}", e);
                Err(e)
            }
        }
    }

    async fn update_metadata(&self) {
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

    async fn create_identity(&mut self) {
        debug!(
            "creating identity with primary address: {}",
            &self.vrsc_address
        );

        // need to convert keys:
        // call verus client to get 160 bit key of `<sequence>.<edition>.geckotest.vrsctest::nft.json`
        // it actually is unnecessary, because we know that the identity has a namespace in and of itself:
        // - the subid is the sequence number
        // - the series is the currency name
        // - created on either testnet or mainnet (VRSC vs vrsctest)
        // that is enough information to find out where the metadata is, as the metadata file has a tag with
        // the vdxfkey of `<sequence>.<edition>.geckotest.vrsctest::nft.json` and can thus be queried on Arweave.
        let mut identity_builder = Identity::builder();

        // if config is testnet {
        identity_builder.testnet(true);
        // }
        if let Err(e) = identity_builder
            .name(&format!("{}", self.sequence))
            .on_currency_name(&self.edition)
            .add_address(&self.vrsc_address)
            .validate()
        {
            error!("something went wrong while creating the identity: {:?}", e);
            return;
        }

        let identity_result = identity_builder.create_identity().await;
        match identity_result {
            Ok(identity) => {
                info!(
                    "identity `{}` has been created! (txid: {})",
                    identity.name_commitment.namereservation.name, identity.registration_txid
                );
                self.identity = Some(identity);
            }
            Err(e) => {
                error!("something went wrong: {:?}", e)
            }
        }
    }
}

pub struct NFTBuilderError {}
