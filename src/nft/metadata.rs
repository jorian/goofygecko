// The user_id is used as an input to the randomizer function.
// This function will be called when a new member joins the Discord. The Event `GuildMemberAdd` is triggered, after which
// this function is called.
use super::config::{self, Attribute};

use indexmap::IndexMap;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand_pcg::{Lcg64Xsh32, Pcg32};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};
use tracing::debug;

pub async fn generate(user_id: u64, config_location: &Path) /* -> NFTMetadata */
{
    // TODO config location error handling
    let asset_config = config::parse(
        config_location
            .to_str()
            .expect("invalid asset_config location"),
    )
    .expect("Error parsing config");

    let output_directory = Path::new("./generated");
    if !output_directory.exists() {
        std::fs::create_dir(output_directory).unwrap(); //TODO catch error
    }
    generate_attributes(user_id, &asset_config, &output_directory).await;
}

async fn generate_attributes(user_id: u64, config: &config::Config, output_directory: &Path) {
    let mut attributes = Vec::new();

    // REMINDER: the rng is deterministic
    let mut rng = Pcg32::seed_from_u64(user_id);

    for (attribute_name, keys) in &config.attributes {
        let mut subattribute: IndexMap<String, f32> = IndexMap::new();

        for (raw_key, a) in keys {
            match a {
                Attribute::Keyed(a) => {
                    if raw_key == "_" {
                        continue;
                    }

                    let good_match = raw_key.split("|").all(|k| {
                        let (key, value) = k.split_once(":").unwrap_or(("_key", k));

                        if attributes
                            .iter()
                            .any(|t: &Trait| t.trait_type == key && t.value == value)
                        {
                            return true;
                        } else {
                            return false;
                        }
                    });

                    if good_match {
                        subattribute = a.clone();
                        break;
                    }
                }
                Attribute::Standard(_) => continue,
            }
        }

        if subattribute.is_empty() {
            for (k, a) in keys {
                match a {
                    Attribute::Keyed(_) => continue,
                    Attribute::Standard(v) => subattribute.insert(k.to_string(), *v),
                };
            }
        }

        calculate_rng_for_attribute(attribute_name, &subattribute, &mut attributes, &mut rng);
    }

    create_metadata(user_id, attributes, config, output_directory)
}

fn calculate_rng_for_attribute(
    attribute_name: &String,
    attribute: &IndexMap<String, f32>,
    attributes: &mut Vec<Trait>,
    rng: &mut Lcg64Xsh32,
) {
    let choices: Vec<&String> = attribute.keys().collect();
    let weights: Vec<&f32> = attribute.values().collect();

    debug!("choices: {:?}", choices);
    debug!("weights: {:?}", weights);

    let dist = WeightedIndex::new(weights)
        .expect("Could not create weighted index, are any odds less than 0?");

    let result = dist.sample(rng);
    dbg!(result);

    // Remove file extension (.png)
    let name = choices[result]
        .strip_suffix(".png")
        .unwrap_or(choices[result]);

    dbg!(name);

    attributes.push(Trait {
        trait_type: attribute_name.to_string(),
        value: name.to_string(),
    });
}

fn create_metadata(
    id: u64,
    mut attributes: Vec<Trait>,
    config: &config::Config,
    output_directory: &Path,
) {
    let image_name = &format!("{}.png", id); //TODO this should be atomic sequential number
    let generated_metadata = NFTMetadata {
        name: &format!("{} #{}", &config.name, id), // TODO this should be atomic sequential number
        identity: &config.identity,
        description: &config.description,
        image: image_name,
        edition: 0,
        attributes: attributes
            .drain(..)
            .filter(|attribute| !attribute.trait_type.starts_with("_"))
            .collect(),
        properties: Properties {
            files: vec![PropertyFile {
                uri: image_name,
                r#type: "image/png",
            }],
            category: "image",
        },
    };

    write_metadata(
        id,
        &serde_json::to_string(&generated_metadata).expect("Could not serialize generated JSON"),
        output_directory,
    )
}

fn write_metadata(id: u64, data: &str, output_directory: &Path) {
    let path_buffer = Path::new(output_directory).join(format!("{}.json", id));

    let mut file = File::create(&path_buffer).expect(&format!(
        "Could not create file at path {}",
        path_buffer.display()
    ));
    write!(file, "{}", data).expect(&format!(
        "Could not write to file at path {}",
        path_buffer.display()
    ));
}

#[derive(Serialize, Deserialize)]
pub struct NFTMetadata<'a> {
    name: &'a str,
    identity: &'a str,
    description: &'a str,
    image: &'a str,
    edition: u16,
    pub attributes: Vec<Trait>,
    properties: Properties<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
struct Properties<'a> {
    files: Vec<PropertyFile<'a>>,
    category: &'a str,
}

#[derive(Serialize, Deserialize)]
struct PropertyFile<'a> {
    uri: &'a str,
    r#type: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn it_works() {
        for i in 1..=9 {
            generate(20 + i, Path::new("./assets/config.json")).await;
        }
    }
}
