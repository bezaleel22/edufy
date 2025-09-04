mod audit;
mod auth;
mod backup;
mod blog;
mod config;
mod error;
mod handlers;
mod kv;
mod middleware;
mod models;
mod storage;

use crate::backup::BackupService;
use crate::config::AppConfig;
use crate::error::AppResult;
use crate::kv::KvStore;
use sqlx::SqlitePool;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: AppConfig,
    pub kv: KvStore,
}

impl AppState {
    pub fn new(db: SqlitePool, config: AppConfig, kv: KvStore) -> Self {
        Self { db, config, kv }
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::from_env()?;
    tracing::info!(
        "Loaded configuration for environment: {}",
        config.environment
    );

    // Initialize database with WAL mode and performance optimizations
    let db = initialize_database(&config.database_url).await?;
    tracing::info!("Connected to database with WAL mode enabled");

    // Run migrations
    sqlx::migrate!("./migrations").run(&db).await?;
    tracing::info!("Database migrations completed");

    // Initialize KV store with proper storage directory
    let kv_storage_dir = if config.environment == "development" {
        "kv_storage"
    } else {
        "/tmp/kv_storage" // For production container
    };
    let kv = KvStore::new(kv_storage_dir)?;
    tracing::info!(
        "KV store initialized for environment: {}",
        config.environment
    );

    // Initialize application state
    let state = AppState::new(db.clone(), config.clone(), kv);

    // Initialize default admin user if in development
    if config.environment == "development" {
        initialize_default_admin(&db).await?;
    }

    // Start backup scheduler if enabled
    if config.backup_enabled {
        start_backup_scheduler(db.clone(), config.clone()).await?;
    }

    // Create router
    let app = handlers::create_router(state);

    // Start server
    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn initialize_default_admin(db: &SqlitePool) -> AppResult<()> {
    use crate::models::UserRole;

    // Check if admin user already exists
    let existing_admin: Option<String> = sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
        .bind("admin@llaweb.com")
        .fetch_optional(db)
        .await?;

    if existing_admin.is_some() {
        tracing::info!("Default admin user already exists");
        return Ok(());
    }

    // Create default admin user (for development only)
    let admin_user = models::User::new(
        "admin@llaweb.com".to_string(),
        UserRole::Admin,
        Some("Administrator".to_string()),
    );

    sqlx::query("INSERT INTO users (id, email, role, google_id, full_name, created_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&admin_user.id)
        .bind(&admin_user.email)
        .bind(&admin_user.role)
        .bind(&admin_user.google_id)
        .bind(&admin_user.full_name)
        .bind(&admin_user.created_at)
        .execute(db)
        .await?;

    tracing::info!("Created default admin user: admin@llaweb.com");
    Ok(())
}

/// Initialize SQLite database with WAL mode and performance optimizations
async fn initialize_database(database_url: &str) -> AppResult<SqlitePool> {
    use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
    use std::str::FromStr;

    // Parse the database URL and configure options
    let options = SqliteConnectOptions::from_str(database_url)?
        .journal_mode(SqliteJournalMode::Wal) // Enable WAL mode
        .synchronous(SqliteSynchronous::Normal) // PRAGMA synchronous = NORMAL
        .create_if_missing(true);

    // Create connection pool
    let pool = SqlitePool::connect_with(options).await?;

    // Apply additional performance pragmas
    sqlx::query("PRAGMA cache_size = -65536") // 64MB page cache
        .execute(&pool)
        .await?;

    sqlx::query("PRAGMA temp_store = MEMORY") // Store temp tables in memory
        .execute(&pool)
        .await?;

    sqlx::query("PRAGMA mmap_size = 268435456") // 256MB memory-mapped I/O
        .execute(&pool)
        .await?;

    sqlx::query("PRAGMA optimize") // Optimize query planner statistics
        .execute(&pool)
        .await?;

    tracing::info!("SQLite configured with WAL mode and performance optimizations");
    Ok(pool)
}

/// Start the backup scheduler
async fn start_backup_scheduler(db: SqlitePool, config: AppConfig) -> AppResult<()> {
    let scheduler = JobScheduler::new().await?;

    let backup_service = BackupService::new(db, config.clone());
    let schedule = config.backup_schedule.clone();

    let job = Job::new_async(schedule.as_str(), move |_uuid, _l| {
        let backup_service = backup_service.clone();
        Box::pin(async move {
            if let Err(e) = backup_service.backup_database().await {
                tracing::error!("Backup failed: {}", e);
            }

            // Also cleanup old SharePoint backups
            if let Err(e) = backup_service.cleanup_old_sharepoint_backups().await {
                tracing::error!("Failed to cleanup old SharePoint backups: {}", e);
            }
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    tracing::info!("Backup scheduler started with schedule: {}", schedule);
    Ok(())
}
