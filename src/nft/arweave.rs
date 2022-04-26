// Handles connection and communication to and from Arweave

use std::path::Path;

use crate::nft::NFTMetadata;

pub fn upload_image(image_location: &Path) -> Result<String, ()> {
    Ok(String::new())
}

pub fn upload_metadata(metadata: &NFTMetadata) -> Result<(), ()> {
    Ok(())
}
