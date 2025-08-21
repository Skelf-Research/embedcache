//! Example of using embedcache as a library
//!
//! This example demonstrates how to use the embedcache library to process text
//! and generate embeddings without running the full web service.
//!
//! Note: This example is meant to show the API structure rather than to be run directly,
//! as it would require setting up the full application state.

use embedcache::{InputDataText, get_default_config};

fn main() {
    // Example text to embed
    let texts = vec![
        "This is an example sentence.".to_string(),
        "Another example sentence for embedding.".to_string(),
    ];

    // Create input data with default configuration
    let _input_data = InputDataText {
        text: texts,
        config: Some(get_default_config()),
    };

    // Note: To actually run this example, you would need to set up the AppState
    // with the models and other components as shown in the main server code.
    //
    // This example is meant to show the API structure rather than to be run directly.

    println!("This is an example of how to use embedcache as a library.");
    println!("In a real application, you would need to set up the models and state.");
}