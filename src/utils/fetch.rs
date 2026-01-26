//! Content fetching utilities

use anyhow::{Context, Result};
use readability::extractor;
use tokio::task;

/// Fetch content from a URL
///
/// This function extracts text content from a URL using the readability library.
pub async fn fetch_content(url: String) -> Result<String> {
    task::spawn_blocking(move || {
        extractor::scrape(&url)
            .map(|product| product.content)
            .unwrap_or_else(|_| String::from("Failed to scrape content"))
    })
    .await
    .context("Failed to fetch content")
}
