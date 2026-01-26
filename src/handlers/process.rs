//! URL processing handler

use actix_web::{web, HttpResponse};
use apistos::api_operation;
use std::collections::HashMap;

use crate::cache::{cache_result, get_from_cache};
use crate::embedding::{Embedder, FastEmbedder};
use crate::models::{get_default_config, AppState, InputData, ProcessedContent};
use crate::utils::{fetch_content, generate_hash};

/// Process a URL and return processed content
///
/// This function fetches content from a URL, chunks it, and generates embeddings.
#[api_operation(summary = "Process a URL and return processed content")]
pub async fn process_url(
    input: web::Json<InputData>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let config = input.config.clone().unwrap_or_else(get_default_config);
    let hash = generate_hash(&input.url, &config);

    // Check cache
    if let Some(cached_content) = get_from_cache(&data.db_pool, hash.clone()).await? {
        return Ok(HttpResponse::Ok().json(cached_content));
    }

    // Fetch content
    let content = fetch_content(input.url.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if content == "Failed to scrape content" {
        let processed_content = ProcessedContent {
            url: input.url.clone(),
            config: config.clone(),
            chunks: HashMap::new(),
            embeddings: HashMap::new(),
            error: Some("Failed to scrape content".to_string()),
        };
        return Ok(HttpResponse::Ok().json(processed_content));
    }

    // Process content
    let chunker = data.chunkers.get(&config.chunking_type).ok_or_else(|| {
        actix_web::error::ErrorBadRequest(format!(
            "Unsupported chunking type: {}",
            config.chunking_type
        ))
    })?;
    let chunks = chunker.chunk(&content, config.chunking_size).await;

    let options = data.models.get(&config.embedding_model).ok_or_else(|| {
        actix_web::error::ErrorBadRequest(format!(
            "Unsupported embedding model: {}",
            config.embedding_model
        ))
    })?;
    let embedder = FastEmbedder {
        options: options.clone(),
    };
    let embeddings = embedder
        .embed(&chunks)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let processed_content = ProcessedContent {
        url: input.url.clone(),
        config: config.clone(),
        chunks: chunks.into_iter().enumerate().collect(),
        embeddings: embeddings.into_iter().enumerate().collect(),
        error: None,
    };

    // Cache result
    cache_result(&data.db_pool, hash.clone(), &processed_content).await?;

    Ok(HttpResponse::Ok().json(processed_content))
}
