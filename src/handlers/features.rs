//! Supported features handler

use actix_web::HttpResponse;
use apistos::api_operation;
use serde_json::json;

use crate::embedding::SUPPORTED_MODELS;

/// List supported features
///
/// This function returns a list of supported chunking types and embedding models.
#[api_operation(summary = "Get a list of supported features")]
pub async fn list_supported_features() -> HttpResponse {
    let supported_features = json!({
        "chunking_types": ["words", "llm-concept", "llm-introspection"],
        "embedding_models": SUPPORTED_MODELS
    });

    HttpResponse::Ok().json(supported_features)
}
