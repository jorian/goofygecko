use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::BTreeMap, fs};

pub fn parse(location: &str) -> Result<Config> {
    let config_file = fs::read_to_string(location).expect("Could not read configuration file");
    let config: Config = serde_json::from_str(&config_file)?;
    Ok(config)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub name: String,
    pub identity: String,
    pub description: String,
    pub attributes: IndexMap<String, BTreeMap<String, Attribute>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Collection {
    pub name: String,
    pub family: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attribute {
    Keyed(IndexMap<String, f32>),
    Standard(f32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Creator {
    pub address: String,
    pub share: u8,
}
