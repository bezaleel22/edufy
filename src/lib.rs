pub mod audit;
pub mod auth;
pub mod backup;
pub mod blog;
pub mod config;
pub mod error;
pub mod handlers;
pub mod kv;
pub mod middleware;
pub mod models;
pub mod storage;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: config::AppConfig,
    pub kv: kv::KvStore,
}

impl AppState {
    pub fn new(db: SqlitePool, config: config::AppConfig, kv: kv::KvStore) -> Self {
        Self { db, config, kv }
    }
}
