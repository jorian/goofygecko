use arloader::{
    transaction::{Base64, FromUtf8Strs, Tag},
    Arweave,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, CACHE_CONTROL},
    Method, Response,
};
use std::path::{Path, PathBuf};
use tracing::debug;
use url::Url;

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
            let signed_tx = self.arweave.sign_transaction(tx).unwrap();

            debug!("signed txid: {}", &signed_tx.id.to_string());

            let tx = self.arweave.post_transaction(&signed_tx).await.unwrap();

            self.file_location = Some(file_location.into());
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

pub async fn get_transaction_by_gecko_number(gecko_number: i64) -> String {
    let identity_name = format!("{}.geckotest@", gecko_number);

    let query = format!(
        r#"
    query {{
      transactions(
        owners:["{}"],
        tags: [
            {{
                name: "vdxfid",
                values: ["{}"]
            }},
            {{
                name: "Content-Type",
                values: ["application/json"]
            }}
        ]
      ) {{
        edges {{
          node {{
            id
          }}
        }}
      }}
    }}"#,
        "8YMcuOtpKW9bAh3Jhzd5Pm9m3kw-32NJSr9M_YBP5pY", identity_name,
    );

    println!("{}", &query);

    let client = gql_client::Client::new("https://arweave.net/graphql");
    let data = client
        .query_unwrap::<serde_json::Value>(&query)
        .await
        .unwrap();

    debug!("graphql response: {:#?}", &data);

    let txid = &data["transactions"]["edges"]
        .as_array()
        .unwrap()
        .first()
        .unwrap()["node"]["id"];

    println!("{}", txid);

    txid.to_string()
}

// todo: split in request and metadata parsing
pub async fn get_metadata_json<'a>(tx_id: &'a str) -> Result<serde_json::Value, ArweaveError> {
    // first check for status. If unconfirmed, return error
    // then get data, it should exist since it was confirmed, but could still go wrong of course.
    let _ = get_transaction_confirmations(tx_id).await?;

    // at this point we know the arweave tx is confirmed.
    debug!("getting metadata");

    let res = req(&format!(
        "https://arweave.net/tx/{}/data",
        tx_id.trim_matches('"')
    ))
    .await?;
    let base64_data = res.text().await?;
    debug!("base64_data: {:?}", base64_data);
    if base64_data.is_empty() {
        return Err(ErrorKind::NoData.into());
    }
    let json_text = base64_url::decode(&base64_data)?;

    debug!("decoded base64 data text: {:?}", json_text);

    Ok(serde_json::from_slice(&json_text)?)
}

pub async fn get_transaction_status(txid: &str) -> Result<serde_json::Value, ArweaveError> {
    debug!("getting arweave transaction status for {}", txid);

    if let Ok(res) = req(&format!(
        "https://arweave.net/tx/{}/status",
        txid.trim_matches('"')
    ))
    .await
    {
        let json = res.json().await?;

        Ok(json)
    } else {
        Err(ErrorKind::NoData.into())
    }
}

pub async fn get_transaction_confirmations(txid: &str) -> Result<i64, ArweaveError> {
    let transaction_status = get_transaction_status(txid).await?;
    debug!("transaction status: {:?}", &transaction_status);

    if let Some(confs) = transaction_status["number_of_confirmations"].as_i64() {
        Ok(confs)
    } else {
        debug!("not valid json: {:#?}", transaction_status);
        return Err(ErrorKind::InvalidJson(String::from(
            "Expected key 'number_of_confirmations' and i64 as value",
        ))
        .into());
    }
}

pub async fn req(url: &str) -> Result<Response, ArweaveError> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CACHE_CONTROL, HeaderValue::from_str("no-cache").unwrap());

    client
        .request(Method::GET, url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.into())
}

#[derive(Debug, Display)]
#[display(fmt = "{}", kind)]
pub struct ArweaveError {
    pub kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

#[derive(Debug, Display)]
pub enum ErrorKind {
    InvalidJson(String),
    NotConfirmed,
    NoData,
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
    Base64DecodeError(base64_url::base64::DecodeError),
}

impl From<ErrorKind> for ArweaveError {
    fn from(kind: ErrorKind) -> Self {
        ArweaveError { kind, source: None }
    }
}

impl From<reqwest::Error> for ArweaveError {
    fn from(e: reqwest::Error) -> Self {
        ErrorKind::ReqwestError(e).into()
    }
}

impl From<serde_json::Error> for ArweaveError {
    fn from(e: serde_json::Error) -> Self {
        ErrorKind::JsonError(e).into()
    }
}

impl From<base64_url::base64::DecodeError> for ArweaveError {
    fn from(e: base64_url::base64::DecodeError) -> Self {
        ErrorKind::Base64DecodeError(e).into()
    }
}
