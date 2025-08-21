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

use actix_web::{web, App, HttpServer};
use embedcache::{AppState, ServerConfig, initialize_db_pool, initialize_models, initialize_chunkers, 
                 process_url, embed_text, list_supported_features};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use apistos::{spec::Spec, info::Info, server::Server, app::{BuildConfig, OpenApiWrapper}};
use apistos::{RapidocConfig, RedocConfig, ScalarConfig, SwaggerUIConfig};
use apistos::web::{get, post, resource, scope};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Load configuration
    let config = ServerConfig::from_env().expect("Failed to load configuration");

    let db_pool = initialize_db_pool(&config).await.expect("Failed to initialize database pool");
    let models = initialize_models(&config).expect("Failed to initialize models");
    let chunkers = initialize_chunkers();

    let app_state = web::Data::new(AppState { db_pool, models, chunkers });

    let server_addr = format!("{}:{}", config.host, config.port);
    println!("Starting server at {}", server_addr);

    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "Embedcache API".to_string(),
                description: Some("This is the embed cache API!".to_string()),
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
            .service(scope("/v1")
                .service(resource("/embed").route(post().to(embed_text)))
                .service(resource("/process").route(post().to(process_url)))
                .service(resource("/params").route(get().to(list_supported_features)))
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