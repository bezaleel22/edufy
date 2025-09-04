use crate::audit::AuditService;
use crate::error::{AppError, AppResult};
use crate::kv::{BlogIndexEntry, BlogPostKv, KvStore};
use crate::models::{BlogPost, CreateBlogPostRequest, User};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct BlogService {
    pub kv: KvStore,
    pub db: SqlitePool,
    pub audit: AuditService,
}

impl BlogService {
    pub fn new(kv: KvStore, db: SqlitePool) -> Self {
        let audit = AuditService::new(db.clone());
        Self { kv, db, audit }
    }

    pub async fn create_post(
        &self,
        payload: CreateBlogPostRequest,
        author_id: String,
    ) -> AppResult<BlogPostKv> {
        // Validate input
        self.validate_blog_post_request(&payload)?;
        
        // Create slug from title
        let slug = self.create_slug(&payload.title);

        // Check if post with same slug already exists
        if let Some(_) = self.kv.get_blog_post(&slug).await? {
            return Err(AppError::Conflict(format!("A blog post with slug '{}' already exists", slug)));
        }

        // Create blog post for KV storage
        let blog_post = BlogPostKv {
            id: Uuid::new_v4().to_string(),
            title: payload.title,
            slug: slug.clone(),
            summary: payload.summary.unwrap_or_default(),
            body_html: payload.body_html,
            author_id: author_id.clone(),
            tags: payload.tags,
            date_published: Utc::now().to_rfc3339(),
            visibility: payload.visibility,
            cover_image: payload.cover_image,
            attachments: payload.attachments,
            meta: None,
        };

        // Store in KV
        self.kv.put_blog_post(&slug, &blog_post).await?;

        // Log audit event
        self.log_audit(&author_id, "create_blog_post", Some(blog_post.id.clone()))
            .await?;

        Ok(blog_post)
    }

    pub async fn get_post(&self, slug: &str) -> AppResult<Option<BlogPostKv>> {
        Ok(self.kv.get_blog_post(slug).await?)
    }

    pub async fn get_public_post(&self, slug: &str) -> AppResult<Option<BlogPostKv>> {
        let blog_post = self.kv.get_blog_post(slug).await?;

        match blog_post {
            Some(post) if post.visibility == "public" => Ok(Some(post)),
            Some(_) => Ok(None), // Private post, return None
            None => Ok(None),
        }
    }

    pub async fn update_post(
        &self,
        slug: &str,
        payload: CreateBlogPostRequest,
        author_id: String,
    ) -> AppResult<BlogPostKv> {
        let mut blog_post = self
            .kv
            .get_blog_post(slug)
            .await?
            .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))?;

        // Update blog post fields
        blog_post.title = payload.title;
        blog_post.summary = payload.summary.unwrap_or_default();
        blog_post.body_html = payload.body_html;
        blog_post.tags = payload.tags;
        blog_post.visibility = payload.visibility;
        blog_post.cover_image = payload.cover_image;
        blog_post.attachments = payload.attachments;

        // Store updated post in KV
        self.kv.put_blog_post(slug, &blog_post).await?;

        // Log audit event
        self.log_audit(&author_id, "update_blog_post", Some(blog_post.id.clone()))
            .await?;

        Ok(blog_post)
    }

    pub async fn delete_post(&self, slug: &str, author_id: String) -> AppResult<BlogPostKv> {
        let blog_post = self
            .kv
            .get_blog_post(slug)
            .await?
            .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))?;

        // Delete from KV
        self.kv.delete_blog_post(slug).await?;

        // Log audit event
        self.log_audit(&author_id, "delete_blog_post", Some(blog_post.id.clone()))
            .await?;

        Ok(blog_post)
    }

    pub async fn list_posts(&self, include_private: bool) -> AppResult<Vec<BlogIndexEntry>> {
        let blog_index = self.kv.get_blog_index().await?;

        if include_private {
            Ok(blog_index)
        } else {
            // Filter out private posts for public access
            let public_posts: Vec<BlogIndexEntry> = blog_index
                .into_iter()
                .filter(|post| post.visibility == "public")
                .collect();
            Ok(public_posts)
        }
    }

    pub async fn get_user(&self, user_id: &str) -> AppResult<Option<User>> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, email, role, google_id, full_name, created_at FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, email, role, google_id, full_name, created_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await?;
        Ok(user)
    }

    fn validate_blog_post_request(&self, payload: &CreateBlogPostRequest) -> AppResult<()> {
        // Validate title
        if payload.title.trim().is_empty() {
            return Err(AppError::Validation("Title cannot be empty".to_string()));
        }
        
        if payload.title.len() > 200 {
            return Err(AppError::Validation("Title cannot exceed 200 characters".to_string()));
        }
        
        // Validate body
        if payload.body_html.trim().is_empty() {
            return Err(AppError::Validation("Content cannot be empty".to_string()));
        }
        
        if payload.body_html.len() > 1_000_000 {
            return Err(AppError::Validation("Content cannot exceed 1MB".to_string()));
        }
        
        // Validate visibility
        if !["public", "private"].contains(&payload.visibility.as_str()) {
            return Err(AppError::Validation("Visibility must be 'public' or 'private'".to_string()));
        }
        
        // Validate tags
        if payload.tags.len() > 10 {
            return Err(AppError::Validation("Cannot have more than 10 tags".to_string()));
        }
        
        for tag in &payload.tags {
            if tag.trim().is_empty() {
                return Err(AppError::Validation("Tags cannot be empty".to_string()));
            }
            if tag.len() > 50 {
                return Err(AppError::Validation("Tag cannot exceed 50 characters".to_string()));
            }
        }
        
        Ok(())
    }

    fn create_slug(&self, title: &str) -> String {
        title
            .to_lowercase()
            .replace(" ", "-")
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' {
                    c
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }

    /// Create a blog post using the BlogPost model constructor
    pub async fn create_post_with_model(
        &self,
        title: String,
        content: String,
        excerpt: String,
        author_id: String,
        tags: Vec<String>,
        visibility: String,
        cover_image: Option<String>,
        inline_images: Vec<String>,
        attachments: Vec<String>,
    ) -> AppResult<BlogPost> {
        // Use BlogPost::new to create the post
        let blog_post = BlogPost::new(
            title,
            content,
            excerpt,
            author_id.clone(),
            tags,
            visibility,
            cover_image,
            inline_images,
            attachments,
        );

        // Check if post with same slug already exists
        if let Some(_) = self.kv.get_blog_post(&blog_post.slug).await? {
            return Err(AppError::Conflict(format!("A blog post with slug '{}' already exists", blog_post.slug)));
        }

        // Convert to KV format for storage
        let blog_post_kv = BlogPostKv {
            id: blog_post.id.clone(),
            title: blog_post.title.clone(),
            slug: blog_post.slug.clone(),
            summary: blog_post.excerpt.clone(),
            body_html: blog_post.content.clone(),
            author_id: blog_post.author_id.clone(),
            tags: blog_post.tags.clone(),
            date_published: blog_post.created_at.to_rfc3339(),
            visibility: blog_post.visibility.clone(),
            cover_image: blog_post.cover_image.clone(),
            attachments: blog_post.attachments.clone(),
            meta: None,
        };

        // Store in KV
        self.kv.put_blog_post(&blog_post.slug, &blog_post_kv).await?;

        // Log audit event
        self.log_audit(&author_id, "create_blog_post_with_model", Some(blog_post.id.clone()))
            .await?;

        Ok(blog_post)
    }

    /// Update a blog post using the BlogPost model update method
    pub async fn update_post_with_model(
        &self,
        slug: &str,
        title: String,
        content: String,
        excerpt: String,
        tags: Vec<String>,
        visibility: String,
        cover_image: Option<String>,
        inline_images: Vec<String>,
        attachments: Vec<String>,
        author_id: String,
    ) -> AppResult<BlogPost> {
        // Get existing post from KV
        let existing_kv = self
            .kv
            .get_blog_post(slug)
            .await?
            .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))?;

        // Convert KV post to BlogPost model
        let mut blog_post = BlogPost {
            id: existing_kv.id,
            slug: existing_kv.slug.clone(),
            title: existing_kv.title,
            content: existing_kv.body_html,
            excerpt: existing_kv.summary,
            author_id: existing_kv.author_id,
            tags: existing_kv.tags,
            visibility: existing_kv.visibility,
            cover_image: existing_kv.cover_image,
            inline_images: vec![], // KV doesn't store inline_images separately
            attachments: existing_kv.attachments,
            created_at: chrono::DateTime::parse_from_rfc3339(&existing_kv.date_published)
                .unwrap_or_else(|_| chrono::Utc::now().into())
                .with_timezone(&chrono::Utc),
            updated_at: chrono::Utc::now(),
        };

        // Use BlogPost::update method to update the post
        blog_post.update(
            title,
            content,
            excerpt,
            tags,
            visibility,
            cover_image,
            inline_images,
            attachments,
        );

        // Convert back to KV format for storage
        let blog_post_kv = BlogPostKv {
            id: blog_post.id.clone(),
            title: blog_post.title.clone(),
            slug: blog_post.slug.clone(),
            summary: blog_post.excerpt.clone(),
            body_html: blog_post.content.clone(),
            author_id: blog_post.author_id.clone(),
            tags: blog_post.tags.clone(),
            date_published: blog_post.created_at.to_rfc3339(),
            visibility: blog_post.visibility.clone(),
            cover_image: blog_post.cover_image.clone(),
            attachments: blog_post.attachments.clone(),
            meta: None,
        };

        // Store updated post in KV
        self.kv.put_blog_post(slug, &blog_post_kv).await?;

        // Log audit event
        self.log_audit(&author_id, "update_blog_post_with_model", Some(blog_post.id.clone()))
            .await?;

        Ok(blog_post)
    }

    async fn log_audit(
        &self,
        user_id: &str,
        action: &str,
        resource_id: Option<String>,
    ) -> AppResult<()> {
        self.audit
            .log_action(user_id, action.to_string(), resource_id, None)
            .await
    }
}
