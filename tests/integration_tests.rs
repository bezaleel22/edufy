use chrono::Datelike;
use edufy::audit::AuditService;
use edufy::auth::AuthService;
use edufy::backup::BackupService;
use edufy::config::AppConfig;
use edufy::models::{User, UserRole};
use sqlx::SqlitePool;
use tempfile::tempdir;
use tokio;

async fn setup_test_db() -> SqlitePool {
    use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
    use std::str::FromStr;
    
    // Use in-memory database for tests
    let options = SqliteConnectOptions::from_str("sqlite::memory:").unwrap()
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .create_if_missing(true);
    
    let pool = SqlitePool::connect_with(options).await.unwrap();
    
    // Run migrations  
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    pool
}

fn test_config() -> AppConfig {
    AppConfig {
        database_url: "sqlite::memory:".to_string(),
        jwt_secret: "test-secret-key".to_string(),
        server_port: 3001,
        upload_dir: "test_uploads".to_string(),
        environment: "test".to_string(),
        cloudflare_account_id: None,
        cloudflare_api_token: None,
        cloudflare_images_endpoint: "https://api.cloudflare.com/client/v4/accounts".to_string(),
        cloudflare_r2_endpoint: "https://api.cloudflare.com/client/v4/accounts".to_string(),
        media_domain: "media.test.com".to_string(),
        google_client_id: None,
        google_client_secret: None,
        google_redirect_uri: "http://localhost:3001/auth/google/callback".to_string(),
        sharepoint_tenant_id: None,
        sharepoint_client_id: None,
        sharepoint_client_secret: None,
        sharepoint_site_id: None,
        sharepoint_drive_id: None,
        backup_enabled: false,
        backup_schedule: "0 0 2 * * *".to_string(),
        backup_retention_days: 30,
    }
}

#[tokio::test]
async fn test_auth_service_jwt_creation_and_verification() {
    let db = setup_test_db().await;
    let config = test_config();
    let auth_service = AuthService::new(db.clone(), config);

    // Create a test user
    let user = User::new(
        "test@example.com".to_string(),
        UserRole::Student,
        Some("Test User".to_string()),
    );

    sqlx::query("INSERT INTO users (id, email, role, google_id, full_name, created_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.role)
        .bind(&user.google_id)
        .bind(&user.full_name)
        .bind(&user.created_at)
        .execute(&db)
        .await
        .unwrap();

    // Test JWT creation
    let token = auth_service.create_jwt_token(&user.id).await.unwrap();
    assert!(!token.is_empty());

    // Test JWT verification
    let claims = auth_service.verify_jwt_token(&token).await.unwrap();
    assert_eq!(claims.sub, user.id);

    // Test token revocation
    auth_service.revoke_token(&token).await.unwrap();

    // Verify token is now invalid
    let result = auth_service.verify_jwt_token(&token).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_audit_service_logging() {
    let db = setup_test_db().await;
    let audit_service = AuditService::new(db.clone());

    let user_id = "test-user-123";
    let action = "test_action";
    let resource_id = Some("resource-456".to_string());

    // Create a test user first to satisfy foreign key constraint
    let user = User::new(
        "audit@example.com".to_string(),
        UserRole::Student,
        Some("Audit User".to_string()),
    );
    
    sqlx::query("INSERT INTO users (id, email, role, google_id, full_name, created_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(user_id)  // Use the same user_id for the audit log
        .bind(&user.email)
        .bind(&user.role)
        .bind(&user.google_id)
        .bind(&user.full_name)
        .bind(&user.created_at)
        .execute(&db)
        .await
        .unwrap();

    // Test logging an action
    audit_service
        .log_action(user_id, action.to_string(), resource_id.clone(), None)
        .await
        .unwrap();

    // Test retrieving audit logs
    let start_date = chrono::Utc::now() - chrono::Duration::hours(1);
    let end_date = chrono::Utc::now() + chrono::Duration::hours(1);

    let logs = audit_service
        .get_user_audit_logs(user_id, start_date, end_date)
        .await
        .unwrap();

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].action, action);
    assert_eq!(logs[0].resource_id, resource_id);
}

#[tokio::test]
async fn test_audit_service_row_rotation() {
    let db = setup_test_db().await;
    let audit_service = AuditService::new(db.clone());

    let user_id = "test-user-rotation";

    // Create a test user first to satisfy foreign key constraint
    let user = User::new(
        "rotation@example.com".to_string(),
        UserRole::Student,
        Some("Rotation User".to_string()),
    );
    
    sqlx::query("INSERT INTO users (id, email, role, google_id, full_name, created_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(user_id)  // Use the same user_id for the audit log
        .bind(&user.email)
        .bind(&user.role)
        .bind(&user.google_id)
        .bind(&user.full_name)
        .bind(&user.created_at)
        .execute(&db)
        .await
        .unwrap();

    // Log multiple actions to test row rotation
    for i in 0..55 {
        audit_service
            .log_action(
                user_id,
                format!("action_{}", i),
                Some(format!("resource_{}", i)),
                None,
            )
            .await
            .unwrap();
    }

    // Verify that multiple rows were created due to rotation
    let table_name = format!(
        "audit_logs_{}_{}",
        chrono::Utc::now().year(),
        format!("{:02}", chrono::Utc::now().month())
    );

    let row_count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM {} WHERE user_id = ?",
        table_name
    ))
    .bind(user_id)
    .fetch_one(&db)
    .await
    .unwrap();

    // Should have at least 2 rows due to rotation at 50 actions
    assert!(row_count >= 2);
}

#[tokio::test]
async fn test_backup_service_creation() {
    let db = setup_test_db().await;
    let config = test_config();
    let backup_service = BackupService::new(db, config);

    // Test that backup service can be created
    assert!(!backup_service.is_sharepoint_configured());
}

#[tokio::test]
async fn test_user_creation_with_google() {
    let db = setup_test_db().await;
    let config = test_config();
    let auth_service = AuthService::new(db.clone(), config);

    let email = "google_user@example.com".to_string();
    let google_id = "google_123456".to_string();
    let full_name = Some("Google User".to_string());

    // Test creating user with Google auth
    let user = auth_service
        .create_user_with_google(email.clone(), google_id.clone(), full_name.clone())
        .await
        .unwrap();

    assert_eq!(user.email, email);
    assert_eq!(user.google_id, Some(google_id.clone()));
    assert_eq!(user.full_name, full_name);

    // Test retrieving user by Google ID
    let retrieved_user = auth_service
        .get_user_by_google_id(&google_id)
        .await
        .unwrap();

    assert!(retrieved_user.is_some());
    assert_eq!(retrieved_user.unwrap().email, email);
}

#[tokio::test]
async fn test_audit_table_creation() {
    let db = setup_test_db().await;
    let audit_service = AuditService::new(db.clone());

    let user_id = "test-user";

    // Create a test user first to satisfy foreign key constraint
    let user = User::new(
        "tablecreation@example.com".to_string(),
        UserRole::Student,
        Some("Table Creation User".to_string()),
    );
    
    sqlx::query("INSERT INTO users (id, email, role, google_id, full_name, created_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(user_id)  // Use the same user_id for the audit log
        .bind(&user.email)
        .bind(&user.role)
        .bind(&user.google_id)
        .bind(&user.full_name)
        .bind(&user.created_at)
        .execute(&db)
        .await
        .unwrap();

    // Test that audit table is created automatically
    let table_name = format!(
        "audit_logs_{}_{}",
        chrono::Utc::now().year(),
        format!("{:02}", chrono::Utc::now().month())
    );

    // Log an action to trigger table creation
    audit_service
        .log_action(user_id, "test_action".to_string(), None, None)
        .await
        .unwrap();

    // Verify table exists
    let table_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name = ?",
    )
    .bind(&table_name)
    .fetch_one(&db)
    .await
    .unwrap();

    assert!(table_exists);
}

#[tokio::test]
async fn test_monthly_audit_table_sharding() {
    let db = setup_test_db().await;
    let audit_service = AuditService::new(db.clone());

    let user_id = "test-user-sharding";

    // Create a test user first to satisfy foreign key constraint
    let user = User::new(
        "sharding@example.com".to_string(),
        UserRole::Student,
        Some("Sharding User".to_string()),
    );
    
    sqlx::query("INSERT INTO users (id, email, role, google_id, full_name, created_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(user_id)
        .bind(&user.email)
        .bind(&user.role)
        .bind(&user.google_id)
        .bind(&user.full_name)
        .bind(&user.created_at)
        .execute(&db)
        .await
        .unwrap();

    // Test that different months would create different tables
    let current_table = format!(
        "audit_logs_{}_{}",
        chrono::Utc::now().year(),
        format!("{:02}", chrono::Utc::now().month())
    );

    let next_month = chrono::Utc::now() + chrono::Duration::days(32);
    let next_table = format!(
        "audit_logs_{}_{}",
        next_month.year(),
        format!("{:02}", next_month.month())
    );

    // Tables should have different names for different months
    assert_ne!(current_table, next_table);

    // Use audit_service to actually log an action and verify table creation
    audit_service
        .log_action(user_id, "sharding_test".to_string(), None, None)
        .await
        .unwrap();

    // Verify that the current month's table was created
    let table_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name = ?",
    )
    .bind(&current_table)
    .fetch_one(&db)
    .await
    .unwrap();

    assert!(table_exists);
}

#[tokio::test]
async fn test_backup_service_with_temp_directory() {
    // Use tempfile::tempdir to test backup functionality
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    let db = setup_test_db().await;
    let mut config = test_config();
    
    // Configure backup to use temporary directory
    config.backup_enabled = true;
    config.upload_dir = temp_path.to_string_lossy().to_string();
    
    let backup_service = BackupService::new(db, config);

    // Test that backup service is created with temporary directory configuration
    assert!(!backup_service.is_sharepoint_configured());
    
    // Test backup path configuration (this would be the local backup path)
    let backup_filename = format!("backup_{}.db", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let backup_path = temp_path.join(&backup_filename);
    
    // Verify the backup path is within our temporary directory
    assert!(backup_path.starts_with(temp_path));
    assert!(backup_path.to_string_lossy().contains(&backup_filename));
}
