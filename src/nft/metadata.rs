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

pub async fn generate(user_id: u64, sequence: u64, config_location: &Path) /* -> NFTMetadata */
{
    // TODO config location error handling
    let asset_config = config::parse(
        config_location
            .to_str()
            .expect("invalid asset_config location"),
    )
    .expect("Error parsing asset config");

    let output_directory = Path::new("./generated");
    if !output_directory.exists() {
        std::fs::create_dir(output_directory)
            .expect("the directory `generated` could not be created")
    }
    generate_attributes(user_id, sequence, &asset_config, &output_directory).await;
}

async fn generate_attributes(
    user_id: u64,
    sequence: u64,
    config: &config::Config,
    output_directory: &Path,
) {
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

                        attributes
                            .iter()
                            .any(|t: &Trait| t.trait_type == key && t.value == value)
                    });

                    if good_match {
                        subattribute = a.clone();
                        break;
                    }
                }
                Attribute::Standard(_) => continue,
            }
        }

        // if there is no subattribute, just use the list of the other attributes and skip keyed ones as they
        // didn't match so they don't belong here.
        // the RNG simply didn't pick the keyed attribute (in the Vec<Trait>) so it must select among the standard attributes.
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

    create_metadata(user_id, sequence, attributes, config, output_directory)
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

    let sum_of_weights: f32 = weights.iter().fold(0.0, |acc, x| acc + *x);
    debug!("sum of weights: {}", sum_of_weights);

    let dist = WeightedIndex::new(weights.clone())
        .expect("Could not create weighted index, are any odds less than 0?");

    let result = dist.sample(rng);
    debug!(result);

    // Remove file extension (.png)
    let name = choices[result]
        .strip_suffix(".png")
        .unwrap_or(choices[result]);

    debug!(name);

    let chosen_weight = weights[result];
    // let rarity = chosen_weight * sum_of_weights;

    attributes.push(Trait {
        trait_type: attribute_name.to_string(),
        value: name.to_string(),
        rarity: *chosen_weight,
    });
}

fn create_metadata(
    user_id: u64,
    sequence: u64,
    attributes: Vec<Trait>,
    config: &config::Config,
    output_directory: &Path,
) {
    let image_name = format!("{}.png", user_id);
    // let mut rarity = attributes.product
    let generated_metadata = NFTMetadata {
        name: format!("{} #{}", &config.name, sequence),
        identity: format!("{}.{}", sequence, &config.identity),
        description: config.description.clone(),
        rarity: attributes.iter().fold(1.0, |acc, x| acc * x.rarity),
        image: image_name.clone(),
        edition: 0,
        attributes, //: attributes
        // .drain(..)
        // .filter(|attribute| !attribute.trait_type.starts_with("_"))
        // .collect(),
        properties: Properties {
            files: vec![PropertyFile {
                uri: image_name.clone(),
                r#type: String::from("image/png"),
            }],
            category: String::from("image"),
        },
    };

    write_metadata(
        user_id,
        &serde_json::to_string(&generated_metadata).expect("Could not serialize generated JSON"),
        output_directory,
    );
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
pub struct NFTMetadata {
    pub name: String,
    pub identity: String,
    pub description: String,
    pub rarity: f32,
    pub image: String,
    pub edition: u16,
    pub attributes: Vec<Trait>,
    properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
    pub rarity: f32,
}

#[derive(Serialize, Deserialize)]
struct Properties {
    files: Vec<PropertyFile>,
    category: String,
}

#[derive(Serialize, Deserialize)]
struct PropertyFile {
    uri: String,
    r#type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn it_works() {
        for i in 1..=9 {
            generate(16843548430 + i, i, Path::new("./assets/config.json")).await;
        }
    }
}
