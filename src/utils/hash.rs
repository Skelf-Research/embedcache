//! Hash generation utilities

use sha2::{Digest, Sha256};

use crate::models::Config;

/// Generate a hash for caching purposes
///
/// Creates a SHA-256 hash based on the URL and configuration parameters.
///
/// # Arguments
///
/// * `url` - The URL being processed
/// * `config` - The configuration used for processing
///
/// # Returns
///
/// A hexadecimal string representation of the hash
pub fn generate_hash(url: &str, config: &Config) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url);
    hasher.update(&config.chunking_type);
    hasher.update(config.chunking_size.to_string());
    hasher.update(&config.embedding_model);
    format!("{:x}", hasher.finalize())
}
