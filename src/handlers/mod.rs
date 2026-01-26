//! HTTP request handlers module

mod embed;
mod features;
mod process;

pub use embed::embed_text;
pub use features::list_supported_features;
pub use process::process_url;
