use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::metadata::NFTMetadata;
use std::fs::read_to_string;

// reads the generated metadata JSON files from "./generated"
// collects the .png files from the metadata attributes
// creates the .png image using libvips
// returns the location of the generated .png file.

pub async fn generate(
    user_id: u64,
    assets_directory: &Path,
    output_directory: &Path,
) -> Result<PathBuf, ()> {
    let metadata_location = Path::new(output_directory).join(format!("{}.json", user_id));
    let contents = read_to_string(&metadata_location).expect(&format!(
        "Could not read file contents for file {}",
        &metadata_location.display()
    ));

    let parsed_metadata: NFTMetadata =
        serde_json::from_str(contents.as_ref()).expect("could not parse metadata JSON");

    create_image(
        user_id,
        &parsed_metadata,
        assets_directory,
        // need to canonicalize for linux
        output_directory.canonicalize().unwrap().as_path(),
    );

    Ok(Path::new(&format!("./generated/{}.png", user_id)).into())
}

fn create_image(id: u64, metadata: &NFTMetadata, assets_directory: &Path, output_directory: &Path) {
    let image_path_buffer = Path::new(output_directory).join(format!("{}.png", id));
    let image_path = image_path_buffer.to_str().expect(&format!(
        "Image is not valid path at {}",
        image_path_buffer.display()
    ));

    let mut composite_command = Command::new("vips");
    composite_command.arg("composite");

    let mut layers = vec![];
    for attribute in &metadata.attributes {
        let layer_path_buffer = Path::new(assets_directory)
            .join(attribute.trait_type.clone())
            .join(format!("{}.png", &attribute.value));
        let layer_path = layer_path_buffer.to_str().expect(&format!(
            "Layer is not valid path at {}",
            layer_path_buffer.display()
        ));
        if layer_path_buffer.exists() {
            // if the path does not exist, the attribute was not meant to be a layer in the image; it was solely
            // meant as a selector of subattributes.
            // This means that the assets guide the image, not the metadata.
            layers.push(layer_path.replace(" ", "\\ ")); // Escape spaces in path
        }
    }

    composite_command
        .arg(layers.join(" "))
        .arg(image_path)
        // Use blending mode "source"
        .arg("2")
        .spawn()
        .expect(&format!("Error creating image {}", id))
        .wait()
        .expect(&format!("Error creating image {}", id));

    // image_path_buffer
}
