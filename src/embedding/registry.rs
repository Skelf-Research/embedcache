//! Embedding model registry

use anyhow::Result;
use fastembed::{EmbeddingModel, TextInitOptions};
use std::collections::HashMap;

use crate::config::ServerConfig;

/// List of all supported embedding models
pub const SUPPORTED_MODELS: &[&str] = &[
    "AllMiniLML6V2",
    "AllMiniLML6V2Q",
    "AllMiniLML12V2",
    "AllMiniLML12V2Q",
    "BGEBaseENV15",
    "BGEBaseENV15Q",
    "BGELargeENV15",
    "BGELargeENV15Q",
    "BGESmallENV15",
    "BGESmallENV15Q",
    "NomicEmbedTextV1",
    "NomicEmbedTextV15",
    "NomicEmbedTextV15Q",
    "ParaphraseMLMiniLML12V2",
    "ParaphraseMLMiniLML12V2Q",
    "ParaphraseMLMpnetBaseV2",
    "BGESmallZHV15",
    "MultilingualE5Small",
    "MultilingualE5Base",
    "MultilingualE5Large",
    "MxbaiEmbedLargeV1",
    "MxbaiEmbedLargeV1Q",
];

/// Get embedding model by name
///
/// This function maps a model name string to an EmbeddingModel enum.
pub fn get_embedding_model(model_name: &str) -> Option<EmbeddingModel> {
    match model_name {
        "AllMiniLML6V2" => Some(EmbeddingModel::AllMiniLML6V2),
        "AllMiniLML6V2Q" => Some(EmbeddingModel::AllMiniLML6V2Q),
        "AllMiniLML12V2" => Some(EmbeddingModel::AllMiniLML12V2),
        "AllMiniLML12V2Q" => Some(EmbeddingModel::AllMiniLML12V2Q),
        "BGEBaseENV15" => Some(EmbeddingModel::BGEBaseENV15),
        "BGEBaseENV15Q" => Some(EmbeddingModel::BGEBaseENV15Q),
        "BGELargeENV15" => Some(EmbeddingModel::BGELargeENV15),
        "BGELargeENV15Q" => Some(EmbeddingModel::BGELargeENV15Q),
        "BGESmallENV15" => Some(EmbeddingModel::BGESmallENV15),
        "BGESmallENV15Q" => Some(EmbeddingModel::BGESmallENV15Q),
        "NomicEmbedTextV1" => Some(EmbeddingModel::NomicEmbedTextV1),
        "NomicEmbedTextV15" => Some(EmbeddingModel::NomicEmbedTextV15),
        "NomicEmbedTextV15Q" => Some(EmbeddingModel::NomicEmbedTextV15Q),
        "ParaphraseMLMiniLML12V2" => Some(EmbeddingModel::ParaphraseMLMiniLML12V2),
        "ParaphraseMLMiniLML12V2Q" => Some(EmbeddingModel::ParaphraseMLMiniLML12V2Q),
        "ParaphraseMLMpnetBaseV2" => Some(EmbeddingModel::ParaphraseMLMpnetBaseV2),
        "BGESmallZHV15" => Some(EmbeddingModel::BGESmallZHV15),
        "MultilingualE5Small" => Some(EmbeddingModel::MultilingualE5Small),
        "MultilingualE5Base" => Some(EmbeddingModel::MultilingualE5Base),
        "MultilingualE5Large" => Some(EmbeddingModel::MultilingualE5Large),
        "MxbaiEmbedLargeV1" => Some(EmbeddingModel::MxbaiEmbedLargeV1),
        "MxbaiEmbedLargeV1Q" => Some(EmbeddingModel::MxbaiEmbedLargeV1Q),
        _ => None,
    }
}

/// Initialize embedding models
///
/// This function creates the available embedding models based on the server configuration.
pub fn initialize_models(config: &ServerConfig) -> Result<HashMap<String, TextInitOptions>> {
    let mut models = HashMap::new();

    for name in &config.enabled_models {
        let model_name = get_embedding_model(name).expect("Invalid model name");
        let options = TextInitOptions::new(model_name).with_show_download_progress(true);
        models.insert(name.to_string(), options);
    }

    Ok(models)
}
