use reqwest::{
    header::{HeaderMap, HeaderValue, CACHE_CONTROL},
    Method,
};
use serde_json::json;
/// Handles connection and communication to and from Arweave
use std::path::{Path, PathBuf};
use tracing::debug;
use url::Url;

use arloader::{
    transaction::{Base64, FromUtf8Strs, Tag},
    Arweave,
};

use super::metadata::NFTMetadata;

// first we need to create and sign the transaction for the image.
// that results in a id which we subsequently use in the metadata file.

pub struct ArweaveTransaction {
    keypair_location: PathBuf,
    arweave: Arweave,
    file_location: Option<PathBuf>,
    content_type: Option<String>,
    id: Option<Base64>,
}

impl ArweaveTransaction {
    pub async fn new(keypair_location: &Path) -> Self {
        let arweave = Arweave::from_keypair_path(
            keypair_location.into(),
            Url::parse("https://arweave.net").unwrap(),
        )
        .await
        .unwrap();

        Self {
            keypair_location: keypair_location.into(),
            arweave,
            file_location: None,
            content_type: None,
            id: None,
        }
    }

    pub async fn upload(
        &mut self,
        file_location: &Path,
        tags: Vec<(&str, &str)>,
    ) -> Result<String, ()> {
        let price_terms = self.arweave.get_price_terms(1.5).await.unwrap();
        debug!("price terms: {:?}", &price_terms);

        // let tag: Tag<Base64> = Tag::from_utf8_strs("Content-Type", &content_type).unwrap();
        if let Ok(tx) = self
            .arweave
            .create_transaction_from_file_path(
                file_location.into(),
                Some(
                    tags.into_iter()
                        .map(|(k, v)| Tag::from_utf8_strs(k, v).unwrap())
                        .collect(),
                ),
                None,
                price_terms,
                false,
            )
            .await
        {
            // sign and send the transaction
            let signed_tx = self.arweave.sign_transaction(tx).unwrap();

            debug!("signed txid: {}", &signed_tx.id.to_string());

            let tx = self.arweave.post_transaction(&signed_tx).await.unwrap();

            self.file_location = Some(file_location.into());
            // self.content_type = Some(content_type.into());
            self.id = Some(tx.0.clone());

            return Ok(tx.0.to_string());
        }

        Err(())
    }

    pub async fn status(&self) -> Result<String, ()> {
        if let Some(id) = &self.id {
            return self
                .arweave
                .get_status(id)
                .await
                .map(|status| status.to_string())
                .or(Err(()));
        }

        Err(())
    }
}

pub async fn get_transaction_by_identity(gecko_number: &str) -> String {
    let identity = format!("{}.geckotest@", gecko_number);

    let query = format!(
        r#"
    query {{
      transactions(
        tags: {{
          name: "identity",
          values: ["{}"]
        }}
      ) {{
        edges {{
          node {{
            id
          }}
        }}
      }}
    }}"#,
        identity
    );

    println!("{}", &query);

    let client = gql_client::Client::new("https://arweave.net/graphql");
    let data = client
        .query_unwrap::<serde_json::Value>(&query)
        .await
        .unwrap();

    let txid = &data["transactions"]["edges"]
        .as_array()
        .unwrap()
        .first()
        .unwrap()["node"]["id"];

    println!("{}", txid);

    txid.to_string()
}

pub async fn get_metadata_json<'a>(tx_id: &'a str) -> NFTMetadata {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CACHE_CONTROL, HeaderValue::from_str("no-cache").unwrap());
    let res = client
        .request(
            Method::GET,
            format!("https://arweave.net/tx/{}/data", tx_id),
        )
        .headers(headers)
        .send()
        .await;
    // let res = reqwest::get(format!("https://arweave.net/tx/{}/data", tx_id)).await;

    debug!("res: {:?}", res);

    let base64_data = res.expect("a request").text().await.expect("base64_data");
    debug!("{:?}", base64_data);
    let json_text = base64_url::decode(&base64_data).expect("decoded base64 data");

    debug!("text: {:?}", json_text);

    let metadata: NFTMetadata = serde_json::from_slice(&json_text).expect("NFTMetadata object");
    //json().await.expect("a json");

    metadata
}
