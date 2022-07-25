// Handles connection and communication to and from Arweave

use std::path::Path;

// first we need to create and sign the transaction for the image.
// that results in a id which we subsequently use in the metadata file.
//

pub fn upload_image(image_location: &Path) -> Result<String, ()> {
    Ok(String::new())
}

// pub fn upload_metadata(metadata: &NFTMetadata) -> Result<(), ()> {
//     Ok(())
// }

// struct TransactionBuilder {
//     image_tx_id: Option<String>,
// }
// impl TransactionBuilder {
//     pub async fn new(id: u64) -> Self {
//         let keypair_path = "/home/jorian/.ardrivewallet.json";
//         let arweave = Arweave::from_keypair_path(
//             keypair_path.into(),
//             Url::parse("https://arweave.net").unwrap(),
//         )
//         .await
//         .unwrap();
//     }
// }
