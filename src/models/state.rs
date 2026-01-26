//! Application state

use async_sqlite::Pool;
use fastembed::TextInitOptions;
use std::collections::HashMap;
use crate::chunking::ContentChunker;

/// Application state shared across handlers
///
/// Contains the database pool, loaded models, and chunkers.
/// The chunkers are stored in a HashMap where the key is the chunker name
/// and the value is a boxed trait object implementing the ContentChunker trait.
pub struct AppState {
    pub db_pool: Pool,
    pub models: HashMap<String, TextInitOptions>,
    pub chunkers: HashMap<String, Box<dyn ContentChunker + Send + Sync>>,
}
