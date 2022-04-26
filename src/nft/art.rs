use std::path::{Path, PathBuf};

// reads the generated metadata file
// collects the .png files from the metadata attributes
// creates the .png image using libvips
// returns the location of the generated .png file.

pub fn generate(assets_directory: &Path, output_directory: &Path) -> Result<PathBuf, ()> {
    Ok(Path::new("./generated/0.png").into())
}
