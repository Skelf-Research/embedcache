//! Complete example of using embedcache as a library
//!
//! This example demonstrates how to set up the embedcache library for actual use
//! in another Rust application. It shows how to directly use the embedding models
//! without running the full web service.

use embedcache::{
    FastEmbedder, Embedder
};
use fastembed::{InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an embedder
    let embedder = FastEmbedder {
        options: InitOptions::new(EmbeddingModel::BGESmallENV15),
    };

    // Example texts to embed
    let texts = vec![
        "This is an example sentence.".to_string(),
        "Another example sentence for embedding.".to_string(),
    ];

    // Generate embeddings
    let embeddings = embedder.embed(&texts).await?;

    println!("Generated {} embeddings", embeddings.len());
    for (i, embedding) in embeddings.iter().enumerate() {
        println!("Text {}: First 5 embedding values: {:?}", i, &embedding[..5.min(embedding.len())]);
    }

    Ok(())
}