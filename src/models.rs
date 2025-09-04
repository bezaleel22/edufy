use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// User roles as defined in CMS.md
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserRole {
    Admin,
    Teacher,
    Parent,
    Student,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Teacher => "teacher",
            UserRole::Parent => "parent",
            UserRole::Student => "student",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(UserRole::Admin),
            "teacher" => Some(UserRole::Teacher),
            "parent" => Some(UserRole::Parent),
            "student" => Some(UserRole::Student),
            _ => None,
        }
    }
}

// User model for new passwordless authentication system
#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub role: String,              // Store as string for database compatibility
    pub google_id: Option<String>, // For Google OAuth integration
    pub full_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, role: UserRole, full_name: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            email,
            role: role.as_str().to_string(),
            google_id: None,
            full_name,
            created_at: Utc::now(),
        }
    }

    pub fn new_with_google(
        email: String,
        role: UserRole,
        google_id: String,
        full_name: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            email,
            role: role.as_str().to_string(),
            google_id: Some(google_id),
            full_name,
            created_at: Utc::now(),
        }
    }

    pub fn get_role(&self) -> Option<UserRole> {
        UserRole::from_str(&self.role)
    }
}

// JWT Revocation model for blacklisting tokens
#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct Revocation {
    pub jti: String, // JWT ID
    pub user_id: Option<String>,
    pub revoked_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>, // When the revoked token would have expired
}

impl Revocation {
    pub fn new(jti: String, user_id: Option<String>, expires_at: Option<DateTime<Utc>>) -> Self {
        Self {
            jti,
            user_id,
            revoked_at: Utc::now(),
            expires_at,
        }
    }
}

// New JSON-based audit log model with incremental appends
#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub session_date: String, // Format: YYYY-MM-DD for grouping actions by day
    pub actions: String,      // JSON array of actions
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AuditLog {
    pub fn new(user_id: String, session_date: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            session_date,
            actions: "[]".to_string(), // Start with empty JSON array
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

// Individual audit action for JSON storage
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuditAction {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
}

impl AuditAction {
    pub fn new(
        action: String,
        resource_id: Option<String>,
        details: Option<serde_json::Value>,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            action,
            resource_id,
            details,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlogPost {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub excerpt: String,
    pub author_id: String,
    pub tags: Vec<String>,
    pub visibility: String, // "public" or "private"
    pub cover_image: Option<String>,
    pub inline_images: Vec<String>,
    pub attachments: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BlogPost {
    pub fn new(
        title: String,
        content: String,
        excerpt: String,
        author_id: String,
        tags: Vec<String>,
        visibility: String,
        cover_image: Option<String>,
        inline_images: Vec<String>,
        attachments: Vec<String>,
    ) -> Self {
        let slug = title
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
            .to_string();

        Self {
            id: Uuid::new_v4().to_string(),
            slug,
            title,
            content,
            excerpt,
            author_id,
            tags,
            visibility,
            cover_image,
            inline_images,
            attachments,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update(
        &mut self,
        title: String,
        content: String,
        excerpt: String,
        tags: Vec<String>,
        visibility: String,
        cover_image: Option<String>,
        inline_images: Vec<String>,
        attachments: Vec<String>,
    ) {
        self.title = title;
        self.content = content;
        self.excerpt = excerpt;
        self.tags = tags;
        self.visibility = visibility;
        self.cover_image = cover_image;
        self.inline_images = inline_images;
        self.attachments = attachments;
        self.updated_at = Utc::now();
    }
}

// Request/Response DTOs for passwordless authentication system
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct GoogleAuthRequest {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
    pub expires_at: usize, // Unix timestamp for client-side expiry checks
}

#[derive(Serialize, Clone)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub role: String,
    pub full_name: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateBlogPostRequest {
    pub title: String,
    pub summary: Option<String>,
    pub body_html: String,
    pub tags: Vec<String>,
    pub visibility: String,
    pub cover_image: Option<String>,
    pub attachments: Vec<String>,
}

#[derive(Serialize)]
pub struct BlogPostResponse {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub excerpt: String,
    pub author: UserResponse,
    pub tags: Vec<String>,
    pub visibility: String,
    pub cover_image: Option<String>,
    pub inline_images: Vec<String>,
    pub attachments: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration timestamp
    pub jti: String, // JWT ID for revocation support
    pub iat: usize,  // Issued at timestamp
}
