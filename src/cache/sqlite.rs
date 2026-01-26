//! SQLite-based caching implementation

use anyhow::Result;
use async_sqlite::rusqlite::Error;
use async_sqlite::{JournalMode, Pool, PoolBuilder};

use crate::config::ServerConfig;
use crate::models::ProcessedContent;

/// Initialize the database pool
///
/// This function creates and initializes the database connection pool.
pub async fn initialize_db_pool(config: &ServerConfig) -> Result<Pool> {
    let db_pool = PoolBuilder::new()
        .path(&config.db_path)
        .journal_mode(match config.db_journal_mode.to_lowercase().as_str() {
            "wal" => JournalMode::Wal,
            "truncate" => JournalMode::Truncate,
            "persist" => JournalMode::Persist,
            _ => JournalMode::Wal,
        })
        .open()
        .await
        .expect("Failed to create database pool");

    // Initialize database
    db_pool
        .conn(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS cache (hash TEXT PRIMARY KEY, content TEXT)",
                [],
            )
        })
        .await
        .expect("Failed to create cache table");

    Ok(db_pool)
}

/// Get cached content from the database
///
/// This function retrieves previously processed content from the cache.
pub async fn get_from_cache(
    pool: &Pool,
    hash: String,
) -> Result<Option<ProcessedContent>, actix_web::Error> {
    let result: Option<String> = pool
        .conn(|conn| {
            conn.query_row(
                "SELECT content FROM cache WHERE hash = ?",
                [hash],
                |row| row.get(0),
            )
            .map(Some)
            .or_else(|err| match err {
                Error::QueryReturnedNoRows => Ok(None),
                _ => Err(err),
            })
        })
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result.map(|json| serde_json::from_str(&json).unwrap()))
}

/// Cache processed content in the database
///
/// This function stores processed content in the cache for future use.
pub async fn cache_result(
    pool: &Pool,
    hash: String,
    content: &ProcessedContent,
) -> Result<(), actix_web::Error> {
    let json =
        serde_json::to_string(content).map_err(actix_web::error::ErrorInternalServerError)?;

    pool.conn(|conn| {
        conn.execute(
            "INSERT OR REPLACE INTO cache (hash, content) VALUES (?, ?)",
            [hash, json],
        )
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(())
}
