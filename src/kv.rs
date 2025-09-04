use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

// KV storage implementation that can work with both local development
// and production Cloudflare KV API calls

#[derive(Clone, Debug)]
pub struct KvStore {
    storage_dir: String,
    cloudflare_api_token: Option<String>,
    cloudflare_account_id: Option<String>,
    cloudflare_namespace_id: Option<String>,
    http_client: Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogPostKv {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub body_html: String,
    pub author_id: String,
    pub tags: Vec<String>,
    pub date_published: String,
    pub visibility: String, // "public" | "private"
    pub cover_image: Option<String>,
    pub attachments: Vec<String>,
    pub meta: Option<serde_json::Value>, // optional extra metadata
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogIndexEntry {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub cover_image: Option<String>,
    pub date_published: String,
    pub tags: Vec<String>,
    pub visibility: String,
}

impl KvStore {
    pub fn new(storage_dir: &str) -> Result<Self> {
        // Create storage directory if it doesn't exist
        fs::create_dir_all(storage_dir)?;
        
        Ok(Self {
            storage_dir: storage_dir.to_string(),
            cloudflare_api_token: std::env::var("CLOUDFLARE_API_TOKEN").ok(),
            cloudflare_account_id: std::env::var("CLOUDFLARE_ACCOUNT_ID").ok(),
            cloudflare_namespace_id: std::env::var("CLOUDFLARE_KV_NAMESPACE_ID").ok(),
            http_client: Client::new(),
        })
    }

    fn is_production(&self) -> bool {
        self.cloudflare_api_token.is_some() 
            && self.cloudflare_account_id.is_some() 
            && self.cloudflare_namespace_id.is_some()
    }

    pub async fn put(&self, key: &str, value: &str) -> Result<()> {
        if self.is_production() {
            self.cloudflare_put(key, value).await
        } else {
            self.local_put(key, value).await
        }
    }

    async fn local_put(&self, key: &str, value: &str) -> Result<()> {
        let file_path = Path::new(&self.storage_dir).join(format!("{}.json", key.replace(":", "_")));
        fs::write(file_path, value)?;
        Ok(())
    }

    async fn cloudflare_put(&self, key: &str, value: &str) -> Result<()> {
        let api_token = self.cloudflare_api_token.as_ref().unwrap();
        let account_id = self.cloudflare_account_id.as_ref().unwrap();
        let namespace_id = self.cloudflare_namespace_id.as_ref().unwrap();

        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/values/{}",
            account_id, namespace_id, key
        );

        let response = self.http_client
            .put(&url)
            .header("Authorization", format!("Bearer {}", api_token))
            .header("Content-Type", "text/plain")
            .body(value.to_string())
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("Cloudflare KV PUT failed: {}", error_text));
        }

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        if self.is_production() {
            self.cloudflare_get(key).await
        } else {
            self.local_get(key).await
        }
    }

    async fn local_get(&self, key: &str) -> Result<Option<String>> {
        let file_path = Path::new(&self.storage_dir).join(format!("{}.json", key.replace(":", "_")));
        
        if file_path.exists() {
            let content = fs::read_to_string(file_path)?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    async fn cloudflare_get(&self, key: &str) -> Result<Option<String>> {
        let api_token = self.cloudflare_api_token.as_ref().unwrap();
        let account_id = self.cloudflare_account_id.as_ref().unwrap();
        let namespace_id = self.cloudflare_namespace_id.as_ref().unwrap();

        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/values/{}",
            account_id, namespace_id, key
        );

        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_token))
            .send()
            .await?;

        if response.status().is_success() {
            let content = response.text().await?;
            Ok(Some(content))
        } else if response.status() == 404 {
            Ok(None)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow::anyhow!("Cloudflare KV GET failed: {}", error_text))
        }
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        if self.is_production() {
            self.cloudflare_delete(key).await
        } else {
            self.local_delete(key).await
        }
    }

    async fn local_delete(&self, key: &str) -> Result<()> {
        let file_path = Path::new(&self.storage_dir).join(format!("{}.json", key.replace(":", "_")));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }

    async fn cloudflare_delete(&self, key: &str) -> Result<()> {
        let api_token = self.cloudflare_api_token.as_ref().unwrap();
        let account_id = self.cloudflare_account_id.as_ref().unwrap();
        let namespace_id = self.cloudflare_namespace_id.as_ref().unwrap();

        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/values/{}",
            account_id, namespace_id, key
        );

        let response = self.http_client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", api_token))
            .send()
            .await?;

        if !response.status().is_success() && response.status() != 404 {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("Cloudflare KV DELETE failed: {}", error_text));
        }

        Ok(())
    }

    // Blog-specific methods
    pub async fn put_blog_post(&self, slug: &str, post: &BlogPostKv) -> Result<()> {
        let key = format!("blog:post:{}", slug);
        let value = serde_json::to_string(post)?;
        self.put(&key, &value).await?;

        // Update blog index
        self.update_blog_index(post).await?;
        
        Ok(())
    }

    pub async fn get_blog_post(&self, slug: &str) -> Result<Option<BlogPostKv>> {
        let key = format!("blog:post:{}", slug);
        if let Some(content) = self.get(&key).await? {
            let post: BlogPostKv = serde_json::from_str(&content)?;
            Ok(Some(post))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_blog_post(&self, slug: &str) -> Result<()> {
        let key = format!("blog:post:{}", slug);
        self.delete(&key).await?;

        // Remove from blog index
        self.remove_from_blog_index(slug).await?;
        
        Ok(())
    }

    pub async fn get_blog_index(&self) -> Result<Vec<BlogIndexEntry>> {
        if let Some(content) = self.get("blog:index").await? {
            let index: Vec<BlogIndexEntry> = serde_json::from_str(&content)?;
            Ok(index)
        } else {
            Ok(Vec::new())
        }
    }

    async fn update_blog_index(&self, post: &BlogPostKv) -> Result<()> {
        let mut index = self.get_blog_index().await?;
        
        // Remove existing entry if it exists
        index.retain(|entry| entry.slug != post.slug);
        
        // Add new entry
        let entry = BlogIndexEntry {
            slug: post.slug.clone(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            cover_image: post.cover_image.clone(),
            date_published: post.date_published.clone(),
            tags: post.tags.clone(),
            visibility: post.visibility.clone(),
        };
        
        index.push(entry);
        
        // Sort by date_published (newest first)
        index.sort_by(|a, b| b.date_published.cmp(&a.date_published));
        
        let value = serde_json::to_string(&index)?;
        self.put("blog:index", &value).await?;
        
        Ok(())
    }

    async fn remove_from_blog_index(&self, slug: &str) -> Result<()> {
        let mut index = self.get_blog_index().await?;
        index.retain(|entry| entry.slug != slug);
        
        let value = serde_json::to_string(&index)?;
        self.put("blog:index", &value).await?;
        
        Ok(())
    }
}
