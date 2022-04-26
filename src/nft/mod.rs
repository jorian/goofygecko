/*
Is concerned with:
- image generation
- metadata generation
- metadata retrieval
- rarity generator based on discord id
 */

use std::path::PathBuf;

use tracing::log::error;

mod art;
mod arweave;
mod config;
mod metadata;

pub struct NFTMetadataBuilder {
    assets_directory: PathBuf,
    output_directory: PathBuf,
    config: PathBuf,
    image_path: Option<PathBuf>,
    tx_hash: Option<String>,
}

impl NFTMetadataBuilder {
    pub fn new(name: &str) -> Self {
        NFTMetadataBuilder {
            assets_directory: PathBuf::from("./assets"),
            output_directory: PathBuf::from("./generated"),
            config: PathBuf::from("./assets/config.toml"),
            image_path: None,
            tx_hash: None,
        }
    }

    pub fn generate_metadata(&mut self, user_id: u64) -> &mut Self {
        // generates the metadata attributes partly based on the user_id as a randomizer.
        metadata::generate(user_id);

        self
    }

    pub fn create_image(&mut self) -> &mut Self {
        match art::generate(&self.assets_directory, &self.output_directory) {
            Ok(path) => self.image_path = Some(path),
            Err(e) => error!("Error while generating art: {:?}", e),
        }

        self
    }

    pub fn upload_image(&mut self) -> &mut Self {
        // TODO unwrap we handled an error in `create_image`.
        match arweave::upload_image(self.image_path.as_ref().unwrap()) {
            Ok(hash) => self.tx_hash = Some(hash),
            Err(e) => error!("Error while uploading image to Arweave: {:?}", e),
        }

        self
    }

    pub fn signed_message(&mut self, message: &str) -> &mut Self {
        self
    }

    pub fn verus_id(&mut self, id: &str) -> &mut Self {
        self
    }

    pub fn build(self) -> Result<NFTMetadata, ()> {
        Ok(NFTMetadata {})
    }
}

pub struct NFTMetadata {}

impl NFTMetadata {}
