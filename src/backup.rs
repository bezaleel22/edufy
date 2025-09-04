use crate::config::AppConfig;
use crate::error::{AppError, AppResult};
use chrono::{DateTime, Utc};
use flate2::write::GzEncoder;
use flate2::Compression;
use reqwest::Client;
use serde_json::Value;
use sqlx::SqlitePool;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tokio::fs;

/// Service for managing SQLite backups to SharePoint
#[derive(Clone)]
pub struct BackupService {
    pub db: SqlitePool,
    pub config: AppConfig,
    pub client: Client,
}

impl BackupService {
    pub fn new(db: SqlitePool, config: AppConfig) -> Self {
        let client = Client::new();
        Self { db, config, client }
    }

    /// Perform a full database backup
    pub async fn backup_database(&self) -> AppResult<()> {
        if !self.config.backup_enabled {
            tracing::info!("Backup is disabled in configuration");
            return Ok(());
        }

        tracing::info!("Starting database backup");

        // Create backup directory if it doesn't exist
        let backup_dir = "backups";
        fs::create_dir_all(backup_dir).await?;

        // Generate backup filename with timestamp
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_filename = format!("cms_backup_{}.db", timestamp);
        let backup_path = format!("{}/{}", backup_dir, backup_filename);

        // Perform SQLite backup using VACUUM INTO
        let backup_query = format!("VACUUM INTO '{}'", backup_path);
        sqlx::query(&backup_query).execute(&self.db).await?;

        tracing::info!("Database backup created: {}", backup_path);

        // Compress the backup
        let compressed_path = format!("{}.gz", backup_path);
        self.compress_file(&backup_path, &compressed_path).await?;

        // Remove uncompressed backup
        fs::remove_file(&backup_path).await?;

        // Upload to SharePoint if configured
        if self.is_sharepoint_configured() {
            match self
                .upload_to_sharepoint(&compressed_path, &format!("{}.gz", backup_filename))
                .await
            {
                Ok(_) => {
                    tracing::info!("Backup uploaded to SharePoint successfully");
                    // Remove local compressed backup after successful upload
                    fs::remove_file(&compressed_path).await?;
                }
                Err(e) => {
                    tracing::error!("Failed to upload backup to SharePoint: {}", e);
                    // Keep local backup if SharePoint upload fails
                }
            }
        } else {
            tracing::warn!(
                "SharePoint not configured, keeping local backup: {}",
                compressed_path
            );
        }

        // Clean up old local backups
        self.cleanup_old_local_backups(backup_dir).await?;

        tracing::info!("Database backup completed");
        Ok(())
    }

    /// Compress a file using gzip
    async fn compress_file(&self, input_path: &str, output_path: &str) -> AppResult<()> {
        let mut input_file = File::open(input_path)?;
        let output_file = File::create(output_path)?;
        let mut encoder = GzEncoder::new(output_file, Compression::default());

        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer)?;
        encoder.write_all(&buffer)?;
        encoder.finish()?;

        tracing::info!("Backup compressed: {}", output_path);
        Ok(())
    }

    /// Check if SharePoint is properly configured
    pub fn is_sharepoint_configured(&self) -> bool {
        self.config.sharepoint_tenant_id.is_some()
            && self.config.sharepoint_client_id.is_some()
            && self.config.sharepoint_client_secret.is_some()
            && self.config.sharepoint_site_id.is_some()
            && self.config.sharepoint_drive_id.is_some()
    }

    /// Get access token for MS Graph API
    async fn get_access_token(&self) -> AppResult<String> {
        let tenant_id =
            self.config.sharepoint_tenant_id.as_ref().ok_or_else(|| {
                AppError::Internal("SharePoint tenant ID not configured".to_string())
            })?;
        let client_id =
            self.config.sharepoint_client_id.as_ref().ok_or_else(|| {
                AppError::Internal("SharePoint client ID not configured".to_string())
            })?;
        let client_secret = self
            .config
            .sharepoint_client_secret
            .as_ref()
            .ok_or_else(|| {
                AppError::Internal("SharePoint client secret not configured".to_string())
            })?;

        let token_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            tenant_id
        );

        let params = [
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("scope", "https://graph.microsoft.com/.default"),
            ("grant_type", "client_credentials"),
        ];

        let response = self.client.post(&token_url).form(&params).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::Internal(format!(
                "Failed to get access token: {}",
                error_text
            )));
        }

        let token_response: Value = response.json().await?;
        let access_token = token_response["access_token"]
            .as_str()
            .ok_or_else(|| AppError::Internal("No access token in response".to_string()))?;

        Ok(access_token.to_string())
    }

    /// Upload backup file to SharePoint
    async fn upload_to_sharepoint(&self, file_path: &str, filename: &str) -> AppResult<()> {
        let access_token = self.get_access_token().await?;
        let site_id =
            self.config.sharepoint_site_id.as_ref().ok_or_else(|| {
                AppError::Internal("SharePoint site ID not configured".to_string())
            })?;
        let drive_id =
            self.config.sharepoint_drive_id.as_ref().ok_or_else(|| {
                AppError::Internal("SharePoint drive ID not configured".to_string())
            })?;

        // Read file content
        let file_content = fs::read(file_path).await?;

        // Create folder for backups if it doesn't exist
        let folder_name = "cms_backups";
        let folder_url = format!(
            "https://graph.microsoft.com/v1.0/sites/{}/drives/{}/root:/{}/{}:/content",
            site_id, drive_id, folder_name, filename
        );

        let response = self
            .client
            .put(&folder_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/octet-stream")
            .body(file_content)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::Internal(format!(
                "Failed to upload to SharePoint: {}",
                error_text
            )));
        }

        Ok(())
    }

    /// Clean up old local backup files
    async fn cleanup_old_local_backups(&self, backup_dir: &str) -> AppResult<()> {
        let mut entries = fs::read_dir(backup_dir).await?;
        let cutoff_date = Utc::now() - chrono::Duration::days(7); // Keep local backups for 7 days

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with("cms_backup_") && filename.ends_with(".db.gz") {
                    if let Ok(metadata) = entry.metadata().await {
                        if let Ok(created) = metadata.created() {
                            let created_datetime: DateTime<Utc> = created.into();
                            if created_datetime < cutoff_date {
                                tracing::info!("Removing old local backup: {}", filename);
                                fs::remove_file(&path).await?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Clean up old SharePoint backups
    pub async fn cleanup_old_sharepoint_backups(&self) -> AppResult<()> {
        if !self.is_sharepoint_configured() {
            return Ok(());
        }

        let access_token = self.get_access_token().await?;
        let site_id = self.config.sharepoint_site_id.as_ref().unwrap();
        let drive_id = self.config.sharepoint_drive_id.as_ref().unwrap();

        let folder_url = format!(
            "https://graph.microsoft.com/v1.0/sites/{}/drives/{}/root:/cms_backups:/children",
            site_id, drive_id
        );

        let response = self
            .client
            .get(&folder_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            // Folder might not exist yet
            return Ok(());
        }

        let files_response: Value = response.json().await?;
        let empty_vec = vec![];
        let files = files_response["value"].as_array().unwrap_or(&empty_vec);

        let cutoff_date =
            Utc::now() - chrono::Duration::days(self.config.backup_retention_days as i64);

        for file in files {
            if let Some(name) = file["name"].as_str() {
                if name.starts_with("cms_backup_") && name.ends_with(".db.gz") {
                    if let Some(created_str) = file["createdDateTime"].as_str() {
                        if let Ok(created_date) = DateTime::parse_from_rfc3339(created_str) {
                            let created_utc = created_date.with_timezone(&Utc);
                            if created_utc < cutoff_date {
                                if let Some(file_id) = file["id"].as_str() {
                                    let delete_url = format!(
                                        "https://graph.microsoft.com/v1.0/sites/{}/drives/{}/items/{}",
                                        site_id, drive_id, file_id
                                    );

                                    let delete_response = self
                                        .client
                                        .delete(&delete_url)
                                        .header("Authorization", format!("Bearer {}", access_token))
                                        .send()
                                        .await?;

                                    if delete_response.status().is_success() {
                                        tracing::info!("Deleted old SharePoint backup: {}", name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Restore database from a backup file
    pub async fn restore_database(&self, backup_path: &str) -> AppResult<()> {
        if !Path::new(backup_path).exists() {
            return Err(AppError::Internal(format!(
                "Backup file not found: {}",
                backup_path
            )));
        }

        // This is a simplified restore - in production you'd want more safeguards
        tracing::warn!("Database restore is not implemented yet - manual restore required");
        tracing::info!(
            "To restore manually, replace the database file with: {}",
            backup_path
        );

        Ok(())
    }
}
