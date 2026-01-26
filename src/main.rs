//! EmbedCache - High-performance text embedding service
//!
//! This is the main binary for the embedcache service, which provides a REST API
//! for text chunking and embedding generation with caching capabilities.
//!
//! ## Usage
//!
//! After installation, simply run:
//!
//! ```bash
//! embedcache
//! ```
//!
//! The service will start with the default configuration or the configuration
//! specified in your `.env` file.
//!
//! ## Configuration
//!
//! The service can be configured through environment variables:
//!
//! - `SERVER_HOST`: Server host address (default: 127.0.0.1)
//! - `SERVER_PORT`: Server port (default: 8081)
//! - `DB_PATH`: SQLite database path (default: cache.db)
//! - `DB_JOURNAL_MODE`: SQLite journal mode (default: wal)
//! - `ENABLED_MODELS`: Comma-separated list of enabled models (default: AllMiniLML6V2)
//! - `LLM_PROVIDER`: LLM provider for intelligent chunking (ollama, openai, anthropic)
//! - `LLM_MODEL`: LLM model name (default: llama3)
//! - `LLM_BASE_URL`: LLM API base URL
//! - `LLM_API_KEY`: LLM API key (for OpenAI/Anthropic)
//! - `LLM_TIMEOUT`: LLM request timeout in seconds (default: 60)

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::web::{get, post, resource, scope};
use apistos::{info::Info, server::Server, spec::Spec};
use apistos::{RapidocConfig, RedocConfig, ScalarConfig, SwaggerUIConfig};
use dotenv::dotenv;

use embedcache::{
    embed_text, initialize_chunkers, initialize_db_pool, initialize_models,
    list_supported_features, process_url, AppState, LLMConfig, ServerConfig,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Load configuration
    let config = ServerConfig::from_env().expect("Failed to load configuration");

    let db_pool = initialize_db_pool(&config)
        .await
        .expect("Failed to initialize database pool");
    let models = initialize_models(&config).expect("Failed to initialize models");

    // Initialize LLM config if available
    let llm_config = LLMConfig::from_server_config(&config);
    let chunkers = initialize_chunkers(llm_config.as_ref());

    let app_state = web::Data::new(AppState {
        db_pool,
        models,
        chunkers,
    });

    let server_addr = format!("{}:{}", config.host, config.port);
    println!("Starting server at {}", server_addr);

    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "Embedcache API".to_string(),
                description: Some(
                    "High-performance text embedding API with caching capabilities".to_string(),
                ),
                ..Default::default()
            },
            servers: vec![Server {
                url: "/".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        App::new()
            .document(spec)
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                scope("/v1")
                    .service(resource("/embed").route(post().to(embed_text)))
                    .service(resource("/process").route(post().to(process_url)))
                    .service(resource("/params").route(get().to(list_supported_features))),
            )
            .build_with(
                "/openapi.json",
                BuildConfig::default()
                    .with(RapidocConfig::new(&"/rapidoc"))
                    .with(RedocConfig::new(&"/redoc"))
                    .with(ScalarConfig::new(&"/scalar"))
                    .with(SwaggerUIConfig::new(&"/swagger")),
            )
    })
    .bind(server_addr)?
    .run()
    .await
}
