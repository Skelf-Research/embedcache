//! Text embedding handler

use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::embedding::{Embedder, FastEmbedder};
use crate::models::{get_default_config, AppState, InputDataText};

/// Process text and return embeddings
///
/// This function generates embeddings for a list of text strings.
#[api_operation(summary = "Process a text and return the embeddings")]
pub async fn embed_text(
    input: web::Json<InputDataText>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let config = input.config.clone().unwrap_or_else(get_default_config);
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
        .embed(&input.text)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(embeddings))
}
