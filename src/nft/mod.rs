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

/// an overarching struct that keeps track of all the details:
/// - art
/// - metadata creation and updates
/// - arweave details
/// - identity details

pub struct NFTBuilder {}

impl NFTBuilder {}

pub struct NFTBuilderError {}
