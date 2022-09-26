/*
Is concerned with:
- image generation
- metadata generation
- metadata retrieval
- rarity generator based on discord id
 */

pub(crate) mod art;
pub(crate) mod arweave;
mod config;
pub(crate) mod identity;
pub(crate) mod metadata;

use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
// use serenity::prelude::{Mutex, RwLock};
use crate::configuration::Settings;
use identity::Identity;
use tracing::{debug, error, info};
use vrsc_rpc::{json::vrsc::Address, Auth, Client, RpcApi};

/// an overarching struct that keeps track of all the details when generating a single NFT:
/// - art
/// - metadata creation and updates
/// - arweave details
/// - identity details
///

// static CONFIG_LOCATION: &str = "assets/config.json";
// static ASSETS_LOCATION: &str = "assets/";
// static OUTPUT_LOCATION: &str = "generated/";

#[derive(Debug)]
pub struct VerusNFT {
    // every NFT has its own user_id:
    pub user_id: u64,
    pub vrsc_address: Address,
    pub sequence: u64,
    pub edition: String,
    pub rarity: f64,
    pub generated_image_path: Option<PathBuf>,
    pub generated_metadata_path: Option<PathBuf>,
    pub uploaded_image_tx_hash: Option<String>,
    pub uploaded_metadata_tx_hash: Option<String>,
    pub identity: Option<Identity>,
}

impl VerusNFT {
    pub async fn generate(user_id: u64, app_config: &Settings) -> Self {
        let client = Client::chain("vrsctest", Auth::ConfigFile, None).expect("a verus client");
        let address = client.get_new_address().unwrap();

        let mut nft_builder = Self {
            user_id,
            vrsc_address: address,
            sequence: app_config.application.sequence_start,
            edition: app_config.application.series.clone(),
            rarity: 0.0,
            generated_metadata_path: None,
            generated_image_path: None,
            uploaded_image_tx_hash: None,
            uploaded_metadata_tx_hash: None,
            identity: None,
        };

        let config_location = format!("{}/config.json", &app_config.application.assets_dir);
        nft_builder
            .generate_metadata(&config_location, &app_config.application.output_dir)
            .await;
        nft_builder
            .generate_art(
                &app_config.application.assets_dir,
                &app_config.application.output_dir,
            )
            .await;
        nft_builder
            .arweave_image_upload(&app_config.application.ardrive_wallet_location)
            .await;
        nft_builder.update_metadata().await;
        nft_builder
            .arweave_metadata_upload(&app_config.application.ardrive_wallet_location)
            .await;
        nft_builder.create_identity().await;

        // verus client: get new address
        // store address in database, linked to user (should i do that here??)
        nft_builder
    }

    /// Generates the metadata for the user that just entered and stores it locally.
    async fn generate_metadata(&mut self, config_location: &str, output_location: &str) {
        metadata::generate(self.user_id, self.sequence, Path::new(config_location)).await;

        // TODO this is really ugly, need one source of truth
        // ideally metadata::generate returns the location of the generated metadata, which I can update here:
        let path = PathBuf::from_str(&format!("{}{}.json", output_location, self.user_id))
            .expect("parsing metadata path failed");

        self.generated_metadata_path = Some(path);
    }

    async fn generate_art(&mut self, assets_location: &str, output_location: &str) {
        match art::generate(
            self.user_id,
            Path::new(assets_location),
            Path::new(output_location),
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

    async fn arweave_image_upload(&mut self, arweave_wallet_location: &str) {
        if let Some(path) = self.generated_image_path.clone() {
            let mut arweave_tx =
                arweave::ArweaveTransaction::new(Path::new(arweave_wallet_location)).await;

            debug!("arweave instance created");

            match arweave_tx
                .upload(
                    &path,
                    vec![
                        ("Content-Type", "image/png"),
                        ("identity", &format!("{}.{}@", self.sequence, &self.edition)),
                    ],
                )
                .await
            {
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

    async fn update_metadata(&mut self) {
        // self.metadata
        // self.image_hash
        // put image_hash in metadata.

        // read metadata file
        // update `image` key with actual location on Arweave
        // save metadata file
        if let Some(path) = self.generated_metadata_path.clone() {
            let metadata_file = fs::read_to_string(&path).unwrap();
            let mut metadata: Value = serde_json::from_str(&metadata_file).unwrap();

            self.rarity = metadata
                .get("rarity")
                .expect("rarity to be present")
                .as_f64()
                .expect("a fractional rarity number");

            if let Some(image_hash) = self.uploaded_image_tx_hash.clone() {
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

    async fn arweave_metadata_upload(&mut self, arweave_wallet_location: &str) {
        if let Some(path) = self.generated_metadata_path.clone() {
            let mut arweave_tx =
                arweave::ArweaveTransaction::new(Path::new(arweave_wallet_location)).await;

            debug!("arweave instance created");

            match arweave_tx
                .upload(
                    &path,
                    vec![
                        ("Content-Type", "application/json"),
                        ("vdxfid", &format!("{}.{}@", self.sequence, &self.edition)), //TODO set actual vdxfid
                    ],
                )
                .await
            {
                Ok(tx_hash) => {
                    self.uploaded_metadata_tx_hash = Some(tx_hash);
                }
                Err(e) => {
                    error!("could not upload metadata: {:?}", e);
                }
            }
        } else {
            error!("no generated_metadata_path was found for: {}", self.user_id);
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
        let vdxfkey_hex = "9a55eaaad7bacc9f37a449e315ff32fedc07b126"; //geckotest.vrsctest::nft.json TODO get from vdxfkey call
        let base64_decoded = base64_url::decode(self.uploaded_metadata_tx_hash.as_ref().unwrap());
        let vdxfvalue_hex = hex::encode(base64_decoded.unwrap());

        debug!("vdxfvalue_hex {:?}", vdxfvalue_hex);

        let mut identity_builder = Identity::builder();

        // if config is testnet {
        identity_builder.testnet(true);
        // }
        if let Err(e) = identity_builder
            .name(&format!("{}", self.sequence))
            .on_currency_name(&self.edition)
            .add_address(&self.vrsc_address)
            .with_content_map(json!({ vdxfkey_hex: &vdxfvalue_hex }))
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

#[cfg(test)]
mod tests {
    // use super::*;
    // #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    // async fn test_image_generation() {
    //     let user_id = 138515381;
    //     for i in 1..=9 {
    //         let mut verus_nft = VerusNFT {
    //             user_id: user_id + i,
    //             vrsc_address: Address::from_str("RVcxxLdedtLvysS4vXZitd3TXe6AjU5WEz").unwrap(),
    //             sequence: 7000 + i,
    //             edition: String::from("geckotest"),
    //             rarity: 0.0,
    //             generated_image_path: None,
    //             generated_metadata_path: None,
    //             uploaded_image_tx_hash: None,
    //             uploaded_metadata_tx_hash: None,
    //             identity: None,
    //         };

    //         verus_nft.generate_metadata().await;
    //         verus_nft.generate_art().await;

    //         assert!(Path::new(OUTPUT_LOCATION)
    //             .join(format!("{}.png", user_id + i))
    //             .exists());

    //         assert!(Path::new(OUTPUT_LOCATION)
    //             .join(format!("{}.json", user_id + i))
    //             .exists());
    //     }
    // }

    // #[tokio::test]
    // async fn test_vdxf_id() {
    //     let client = Client::chain("vrsctest", Auth::ConfigFile, None).expect("a verus client");

    //     let vdxfid = client
    //         .get_vdxf_id("geckotest.vrsctest::nft.json", None)
    //         .expect("a vdxfid object");

    //     dbg!(vdxfid);
    // }
}
