use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
    pub upload_dir: String,
    pub environment: String,
    pub cloudflare_account_id: Option<String>,
    pub cloudflare_api_token: Option<String>,
    pub cloudflare_images_endpoint: String,
    pub cloudflare_r2_endpoint: String,
    pub media_domain: String,
    // Google OAuth configuration
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_redirect_uri: String,
    // SharePoint/MS Graph configuration
    pub sharepoint_tenant_id: Option<String>,
    pub sharepoint_client_id: Option<String>,
    pub sharepoint_client_secret: Option<String>,
    pub sharepoint_site_id: Option<String>,
    pub sharepoint_drive_id: Option<String>,
    // Backup configuration
    pub backup_enabled: bool,
    pub backup_schedule: String, // Cron expression
    pub backup_retention_days: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "sqlite:cms.db".to_string(),
            jwt_secret: "your-super-secret-jwt-key-change-in-production".to_string(),
            server_port: 3001,
            upload_dir: "uploads".to_string(),
            environment: "development".to_string(),
            cloudflare_account_id: None,
            cloudflare_api_token: None,
            cloudflare_images_endpoint: "https://api.cloudflare.com/client/v4/accounts".to_string(),
            cloudflare_r2_endpoint: "https://api.cloudflare.com/client/v4/accounts".to_string(),
            media_domain: "media.llacademy.ng".to_string(),
            google_client_id: None,
            google_client_secret: None,
            google_redirect_uri: "http://localhost:3001/auth/google/callback".to_string(),
            sharepoint_tenant_id: None,
            sharepoint_client_id: None,
            sharepoint_client_secret: None,
            sharepoint_site_id: None,
            sharepoint_drive_id: None,
            backup_enabled: false,
            backup_schedule: "0 0 2 * * *".to_string(), // Daily at 2 AM
            backup_retention_days: 30,
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("database_url", "sqlite:cms.db")?
            .set_default(
                "jwt_secret",
                "your-super-secret-jwt-key-change-in-production",
            )?
            .set_default("server_port", 3001)?
            .set_default("upload_dir", "uploads")?
            .set_default("environment", "development")?
            .set_default(
                "cloudflare_images_endpoint",
                "https://api.cloudflare.com/client/v4/accounts",
            )?
            .set_default(
                "cloudflare_r2_endpoint",
                "https://api.cloudflare.com/client/v4/accounts",
            )?
            .set_default("media_domain", "media.llacademy.ng")?
            .set_default("google_redirect_uri", "http://localhost:3001/auth/google/callback")?
            .set_default("backup_enabled", false)?
            .set_default("backup_schedule", "0 0 2 * * *")?
            .set_default("backup_retention_days", 30)?;

        // Override with environment variables if they exist
        if let Ok(db_url) = env::var("DATABASE_URL") {
            builder = builder.set_override("database_url", db_url)?;
        }
        if let Ok(jwt_secret) = env::var("JWT_SECRET") {
            builder = builder.set_override("jwt_secret", jwt_secret)?;
        }
        if let Ok(port) = env::var("PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                builder = builder.set_override("server_port", port_num)?;
            }
        }
        if let Ok(upload_dir) = env::var("UPLOAD_DIR") {
            builder = builder.set_override("upload_dir", upload_dir)?;
        }
        if let Ok(env) = env::var("ENVIRONMENT") {
            builder = builder.set_override("environment", env)?;
        }
        if let Ok(account_id) = env::var("CLOUDFLARE_ACCOUNT_ID") {
            builder = builder.set_override("cloudflare_account_id", account_id)?;
        }
        if let Ok(api_token) = env::var("CLOUDFLARE_API_TOKEN") {
            builder = builder.set_override("cloudflare_api_token", api_token)?;
        }
        if let Ok(images_endpoint) = env::var("CLOUDFLARE_IMAGES_ENDPOINT") {
            builder = builder.set_override("cloudflare_images_endpoint", images_endpoint)?;
        }
        if let Ok(r2_endpoint) = env::var("CLOUDFLARE_R2_ENDPOINT") {
            builder = builder.set_override("cloudflare_r2_endpoint", r2_endpoint)?;
        }
        if let Ok(media_domain) = env::var("MEDIA_DOMAIN") {
            builder = builder.set_override("media_domain", media_domain)?;
        }

        // Google OAuth configuration
        if let Ok(google_client_id) = env::var("GOOGLE_CLIENT_ID") {
            builder = builder.set_override("google_client_id", google_client_id)?;
        }
        if let Ok(google_client_secret) = env::var("GOOGLE_CLIENT_SECRET") {
            builder = builder.set_override("google_client_secret", google_client_secret)?;
        }
        if let Ok(google_redirect_uri) = env::var("GOOGLE_REDIRECT_URI") {
            builder = builder.set_override("google_redirect_uri", google_redirect_uri)?;
        }

        // SharePoint configuration
        if let Ok(sharepoint_tenant_id) = env::var("SHAREPOINT_TENANT_ID") {
            builder = builder.set_override("sharepoint_tenant_id", sharepoint_tenant_id)?;
        }
        if let Ok(sharepoint_client_id) = env::var("SHAREPOINT_CLIENT_ID") {
            builder = builder.set_override("sharepoint_client_id", sharepoint_client_id)?;
        }
        if let Ok(sharepoint_client_secret) = env::var("SHAREPOINT_CLIENT_SECRET") {
            builder = builder.set_override("sharepoint_client_secret", sharepoint_client_secret)?;
        }
        if let Ok(sharepoint_site_id) = env::var("SHAREPOINT_SITE_ID") {
            builder = builder.set_override("sharepoint_site_id", sharepoint_site_id)?;
        }
        if let Ok(sharepoint_drive_id) = env::var("SHAREPOINT_DRIVE_ID") {
            builder = builder.set_override("sharepoint_drive_id", sharepoint_drive_id)?;
        }

        // Backup configuration
        if let Ok(backup_enabled) = env::var("BACKUP_ENABLED") {
            if let Ok(enabled) = backup_enabled.parse::<bool>() {
                builder = builder.set_override("backup_enabled", enabled)?;
            }
        }
        if let Ok(backup_schedule) = env::var("BACKUP_SCHEDULE") {
            builder = builder.set_override("backup_schedule", backup_schedule)?;
        }
        if let Ok(backup_retention_days) = env::var("BACKUP_RETENTION_DAYS") {
            if let Ok(days) = backup_retention_days.parse::<u32>() {
                builder = builder.set_override("backup_retention_days", days)?;
            }
        }

        builder.build()?.try_deserialize()
    }
}
