//! Core data types for embedcache

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use apistos::ApiComponent;
use schemars::JsonSchema;

/// Configuration for text processing
///
/// This struct defines the parameters used for chunking and embedding generation.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct Config {
    pub chunking_type: String,
    pub chunking_size: usize,
    pub embedding_model: String,
}

/// Processed content with embeddings
///
/// This struct contains the results of processing a URL or text, including
/// the chunks and their corresponding embeddings.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ProcessedContent {
    pub url: String,
    pub config: Config,
    pub chunks: HashMap<usize, String>,
    pub embeddings: HashMap<usize, Vec<f32>>,
    pub error: Option<String>,
}

/// Input data for URL processing
#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct InputData {
    pub url: String,
    pub config: Option<Config>,
}

/// Input data for text embedding
#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct InputDataText {
    pub text: Vec<String>,
    pub config: Option<Config>,
}

/// Get the default configuration for text processing
///
/// Returns a Config struct with sensible defaults:
/// - chunking_type: "words"
/// - chunking_size: 512
/// - embedding_model: "BGESmallENV15"
pub fn get_default_config() -> Config {
    Config {
        chunking_type: "words".to_string(),
        chunking_size: 512,
        embedding_model: "BGESmallENV15".to_string(),
    }
}
