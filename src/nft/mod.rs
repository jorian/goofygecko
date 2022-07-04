/*
Is concerned with:
- image generation
- metadata generation
- metadata retrieval
- rarity generator based on discord id
 */

// use std::path::{Path, PathBuf};

// use tracing::log::error;

pub(crate) mod art;
mod arweave;
mod config;
pub(crate) mod metadata;
