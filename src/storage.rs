use axum::extract::Multipart;
use tokio::fs;
use uuid::Uuid;
use crate::error::{AppError, AppResult};
use crate::config::AppConfig;


#[derive(Debug, Clone, serde::Serialize)]
pub struct MediaUploadResult {
    pub url: String,
    pub file_id: String,
    pub file_type: MediaType,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum MediaType {
    Image,
    Document,
    Video,
    Other,
}

impl MediaType {
    pub fn from_content_type(content_type: &str) -> Self {
        match content_type {
            ct if ct.starts_with("image/") => MediaType::Image,
            ct if ct.starts_with("video/") => MediaType::Video,
            "application/pdf" => MediaType::Document,
            ct if ct.contains("document") => MediaType::Document,
            ct if ct.contains("zip") => MediaType::Document,
            _ => MediaType::Other,
        }
    }
}

pub struct MediaUploader {
    config: AppConfig,
}

impl MediaUploader {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Upload image to Cloudflare Images (or local storage in development)
    pub async fn upload_image(&self, file_data: Vec<u8>, filename: &str, content_type: &str) -> AppResult<MediaUploadResult> {
        if self.config.cloudflare_account_id.is_some() && self.config.cloudflare_api_token.is_some() {
            self.upload_to_cloudflare_images(file_data, filename).await
        } else {
            // Development: Save locally
            self.save_image_locally(file_data, filename, content_type).await
        }
    }

    /// Upload file to Cloudflare R2 (or local storage in development)
    pub async fn upload_file(&self, file_data: Vec<u8>, filename: &str, content_type: &str) -> AppResult<MediaUploadResult> {
        if self.config.cloudflare_account_id.is_some() && self.config.cloudflare_api_token.is_some() {
            self.upload_to_cloudflare_r2(file_data, filename, content_type).await
        } else {
            // Development: Save locally
            self.save_file_locally(file_data, filename, content_type).await
        }
    }

    /// Save image locally for development
    async fn save_image_locally(&self, file_data: Vec<u8>, filename: &str, content_type: &str) -> AppResult<MediaUploadResult> {
        let file_id = Uuid::new_v4().to_string();
        let extension = self.get_file_extension(filename, content_type);
        let safe_filename = format!("img-{}.{}", file_id, extension);
        let file_path = format!("{}/images/{}", self.config.upload_dir, safe_filename);
        
        // Create directory if it doesn't exist
        fs::create_dir_all(format!("{}/images", self.config.upload_dir)).await?;
        
        // Save file
        fs::write(&file_path, file_data).await?;
        
        // Return local URL (in production this would be the Cloudflare Images URL)
        let url = format!("http://localhost:{}/uploads/images/{}", self.config.server_port, safe_filename);
        
        Ok(MediaUploadResult {
            url,
            file_id,
            file_type: MediaType::Image,
        })
    }

    /// Save file locally for development
    async fn save_file_locally(&self, file_data: Vec<u8>, filename: &str, content_type: &str) -> AppResult<MediaUploadResult> {
        let file_id = Uuid::new_v4().to_string();
        let extension = self.get_file_extension(filename, content_type);
        let safe_filename = format!("file-{}.{}", file_id, extension);
        let file_path = format!("{}/files/{}", self.config.upload_dir, safe_filename);
        
        // Create directory if it doesn't exist
        fs::create_dir_all(format!("{}/files", self.config.upload_dir)).await?;
        
        // Save file
        fs::write(&file_path, file_data).await?;
        
        // Return local URL (in production this would be the R2 URL)
        let url = format!("http://localhost:{}/uploads/files/{}", self.config.server_port, safe_filename);
        
        Ok(MediaUploadResult {
            url,
            file_id,
            file_type: MediaType::from_content_type(content_type),
        })
    }

    /// Upload to Cloudflare Images (production)
    async fn upload_to_cloudflare_images(&self, file_data: Vec<u8>, filename: &str) -> AppResult<MediaUploadResult> {
        let account_id = self.config.cloudflare_account_id.as_ref()
            .ok_or_else(|| AppError::Internal("Cloudflare account ID not configured".to_string()))?;
        let api_token = self.config.cloudflare_api_token.as_ref()
            .ok_or_else(|| AppError::Internal("Cloudflare API token not configured".to_string()))?;

        let client = reqwest::Client::new();
        let url = format!("{}/{}/images/v1", self.config.cloudflare_images_endpoint, account_id);
        
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(file_data)
                .file_name(filename.to_string()));

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_token))
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await?;
            let image_id = json["result"]["id"].as_str()
                .ok_or_else(|| AppError::Internal("Invalid Cloudflare Images response".to_string()))?;
            
            let optimized_url = format!("https://{}/cf_images/{}", self.config.media_domain, image_id);
            
            Ok(MediaUploadResult {
                url: optimized_url,
                file_id: image_id.to_string(),
                file_type: MediaType::Image,
            })
        } else {
            let error_text = response.text().await?;
            Err(AppError::Internal(format!("Cloudflare Images upload failed: {}", error_text)))
        }
    }

    /// Upload to Cloudflare R2 (production)
    async fn upload_to_cloudflare_r2(&self, _file_data: Vec<u8>, filename: &str, content_type: &str) -> AppResult<MediaUploadResult> {
        let _account_id = self.config.cloudflare_account_id.as_ref()
            .ok_or_else(|| AppError::Internal("Cloudflare account ID not configured".to_string()))?;
        let _api_token = self.config.cloudflare_api_token.as_ref()
            .ok_or_else(|| AppError::Internal("Cloudflare API token not configured".to_string()))?;

        // Generate a unique file path based on content type and date
        let file_id = Uuid::new_v4().to_string();
        let year = chrono::Utc::now().format("%Y").to_string();
        let extension = self.get_file_extension(filename, content_type);
        
        let file_path = match MediaType::from_content_type(content_type) {
            MediaType::Document => format!("reports/{}/{}.{}", year, file_id, extension),
            MediaType::Video => format!("events/{}/{}.{}", year, file_id, extension),
            _ => format!("docs/{}.{}", file_id, extension),
        };

        // In production, you would use AWS SDK or direct R2 API calls
        // For now, we'll simulate this with a placeholder
        let r2_url = format!("https://{}/r2/{}", self.config.media_domain, file_path);
        
        // TODO: Implement actual R2 upload using AWS SDK or direct API calls
        tracing::warn!("R2 upload not implemented yet, using placeholder URL");
        
        Ok(MediaUploadResult {
            url: r2_url,
            file_id,
            file_type: MediaType::from_content_type(content_type),
        })
    }

    /// Process multipart form data and upload files
    pub async fn process_multipart_upload(&self, mut multipart: Multipart) -> AppResult<Vec<MediaUploadResult>> {
        let mut results = Vec::new();

        while let Some(field) = multipart.next_field().await? {
            if let Some(filename) = field.file_name() {
                let filename = filename.to_string();
                let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
                let data = field.bytes().await?;
                
                let result = if MediaType::from_content_type(&content_type) == MediaType::Image {
                    self.upload_image(data.to_vec(), &filename, &content_type).await?
                } else {
                    self.upload_file(data.to_vec(), &filename, &content_type).await?
                };
                
                results.push(result);
            }
        }

        Ok(results)
    }

    fn get_file_extension(&self, filename: &str, content_type: &str) -> String {
        // Try to get extension from filename first
        if let Some(ext) = filename.split('.').last() {
            if ext.len() <= 4 && ext != filename {
                return ext.to_lowercase();
            }
        }
        
        // Fallback to content type
        match content_type {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            "application/pdf" => "pdf",
            "video/mp4" => "mp4",
            "video/webm" => "webm",
            "application/zip" => "zip",
            _ => "bin",
        }.to_string()
    }
}
