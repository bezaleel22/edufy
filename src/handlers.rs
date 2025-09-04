use axum::{
    extract::{Extension, Path, State},
    http::{header, HeaderMap, StatusCode},
    middleware,
    response::{IntoResponse, Json},
    routing::{get, post, put},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::audit::AuditService;
use crate::auth::AuthService;
use crate::backup::BackupService;
use crate::blog::BlogService;
use crate::error::{AppError, AppResult};
use crate::kv::{BlogIndexEntry, BlogPostKv};
use crate::middleware::{admin_middleware, auth_middleware, AuthUser};
use crate::models::{
    AuditAction, CreateBlogPostRequest, GoogleAuthRequest, LoginRequest, User, UserResponse,
};
use crate::storage::MediaUploader;
use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    // Create admin routes with authentication middleware
    let admin_routes = Router::new()
        .route(
            "/api/admin/posts",
            get(admin_list_posts).post(admin_create_post),
        )
        .route(
            "/api/admin/posts/{slug}",
            get(admin_get_post)
                .put(admin_update_post)
                .delete(admin_delete_post),
        )
        .route(
            "/api/admin/audit/logs/{user_id}",
            get(admin_get_user_audit_logs),
        )
        .route(
            "/api/admin/audit/cleanup",
            post(admin_cleanup_old_audit_tables),
        )
        .route("/api/admin/backup/restore", post(admin_restore_database))
        .route("/api/admin/users/{user_id}", get(admin_get_user))
        .route(
            "/api/admin/users/email/{email}",
            get(admin_get_user_by_email),
        )
        .route(
            "/api/admin/users/{user_id}/role/{role}",
            get(admin_check_user_role),
        )
        .route("/api/admin/posts/model", post(admin_create_post_with_model))
        .route(
            "/api/admin/posts/model/{slug}",
            put(admin_update_post_with_model),
        )
        .route("/api/admin/upload/image", post(admin_upload_image))
        .route("/api/admin/upload/file", post(admin_upload_file))
        .route("/api/admin/upload/multipart", post(admin_upload_multipart))
        .layer(middleware::from_fn(admin_middleware))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Create protected routes with authentication middleware
    let protected_routes = Router::new()
        .route("/api/users/me", get(verify_session))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/", get(root))
        .route("/healthz", get(health_check))
        // Auth routes (no middleware needed)
        .route("/api/auth/login", post(login))
        .route("/api/auth/google", post(google_oauth_login))
        .route("/api/auth/logout", post(logout))
        // Public blog endpoints for SvelteKit SSR (no middleware needed)
        .route("/api/blog/index", get(get_blog_index))
        .route("/api/blog/post/{slug}", get(get_public_blog_post))
        .route("/api/blog/public/{slug}", get(get_public_post_direct))
        // Merge protected routes
        .merge(admin_routes)
        .merge(protected_routes)
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "https://llacademy.ng"
                        .parse::<axum::http::HeaderValue>()
                        .unwrap(),
                    "https://www.llacademy.ng"
                        .parse::<axum::http::HeaderValue>()
                        .unwrap(),
                    "http://localhost:5173"
                        .parse::<axum::http::HeaderValue>()
                        .unwrap(), // For development
                ])
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers([
                    header::CONTENT_TYPE,
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::COOKIE,
                    header::SET_COOKIE,
                ])
                .allow_credentials(true),
        )
        .with_state(state)
}

async fn root() -> &'static str {
    "LLA Web CMS API"
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "LLA Web CMS"
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    let auth_service = AuthService::new(state.db.clone(), state.config.clone());
    let (response, cookie_value) = auth_service.login(payload).await?;

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie_value.parse().unwrap());

    Ok((headers, Json(response)))
}

async fn google_oauth_login(
    State(state): State<AppState>,
    Json(payload): Json<GoogleAuthRequest>,
) -> AppResult<impl IntoResponse> {
    let auth_service = AuthService::new(state.db.clone(), state.config.clone());
    let (response, cookie_value) = auth_service.google_oauth_login(payload).await?;

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie_value.parse().unwrap());

    Ok((headers, Json(response)))
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> AppResult<impl IntoResponse> {
    let auth_service = AuthService::new(state.db.clone(), state.config.clone());

    // Extract session cookie
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Parse session cookie
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("session=") {
                    let session_value = &cookie[8..];
                    let clear_cookie = auth_service.logout_with_cookie(session_value).await?;

                    let mut response_headers = HeaderMap::new();
                    response_headers.insert(header::SET_COOKIE, clear_cookie.parse().unwrap());

                    return Ok((response_headers, StatusCode::NO_CONTENT));
                }
            }
        }
    }

    Ok((HeaderMap::new(), StatusCode::NO_CONTENT))
}

// Legacy handlers - these should be removed and replaced with admin handlers

// Admin handlers (protected routes)
async fn admin_list_posts(
    State(state): State<AppState>,
    _user: AuthUser,
) -> AppResult<Json<Vec<BlogIndexEntry>>> {
    // User is already verified by middleware
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let posts = blog_service.list_posts(true).await?; // Include private posts
    Ok(Json(posts))
}

async fn admin_create_post(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateBlogPostRequest>,
) -> AppResult<Json<BlogPostKv>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let blog_post = blog_service.create_post(payload, user.0.id.clone()).await?;
    Ok(Json(blog_post))
}

async fn admin_get_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    _user: AuthUser,
) -> AppResult<Json<BlogPostKv>> {
    // User is already verified by middleware
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let blog_post = blog_service
        .get_post(&slug)
        .await?
        .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))?;

    Ok(Json(blog_post))
}

async fn admin_update_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    user: AuthUser,
    Json(payload): Json<CreateBlogPostRequest>,
) -> AppResult<Json<BlogPostKv>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let blog_post = blog_service
        .update_post(&slug, payload, user.0.id.clone())
        .await?;
    Ok(Json(blog_post))
}

async fn admin_delete_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    user: AuthUser,
) -> AppResult<StatusCode> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    blog_service.delete_post(&slug, user.0.id.clone()).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Session verification - user is already verified by middleware
async fn verify_session(user: AuthUser) -> AppResult<Json<UserResponse>> {
    Ok(Json(user.0))
}

// Public blog endpoints for SvelteKit SSR
async fn get_blog_index(State(state): State<AppState>) -> AppResult<Json<Vec<BlogIndexEntry>>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let public_posts = blog_service.list_posts(false).await?; // Only public posts
    Ok(Json(public_posts))
}

async fn get_public_blog_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<BlogPostKv>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let blog_post = blog_service
        .get_post(&slug)
        .await?
        .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))?;

    // Check if post is public
    if blog_post.visibility != "public" {
        return Err(AppError::NotFound("Blog post not found".to_string()));
    }

    Ok(Json(blog_post))
}

// Direct public post endpoint using BlogService.get_public_post method
async fn get_public_post_direct(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<BlogPostKv>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let blog_post = blog_service
        .get_public_post(&slug)
        .await?
        .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))?;

    Ok(Json(blog_post))
}

// Admin audit handlers
async fn admin_get_user_audit_logs(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    _admin_user: AuthUser,
) -> AppResult<Json<Vec<AuditAction>>> {
    let audit_service = AuditService::new(state.db.clone());

    // Get logs for the last 30 days by default
    let end_date = chrono::Utc::now();
    let start_date = end_date - chrono::Duration::days(30);

    let logs = audit_service
        .get_user_audit_logs(&user_id, start_date, end_date)
        .await?;
    Ok(Json(logs))
}

async fn admin_cleanup_old_audit_tables(
    State(state): State<AppState>,
    _admin_user: AuthUser,
) -> AppResult<Json<serde_json::Value>> {
    let audit_service = AuditService::new(state.db.clone());

    // Cleanup tables older than 3 months (as per architecture design)
    audit_service.cleanup_old_audit_tables().await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Old audit tables cleaned up successfully",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// Admin backup handler
async fn admin_restore_database(
    State(state): State<AppState>,
    _admin_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    let backup_service = BackupService::new(state.db.clone(), state.config.clone());

    // Extract backup path from payload
    let backup_path = payload
        .get("backup_path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("backup_path is required".to_string()))?;

    // Restore database from backup
    backup_service.restore_database(backup_path).await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Database restored successfully",
        "backup_path": backup_path,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// Admin user lookup handlers
async fn admin_get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    _admin_user: AuthUser,
) -> AppResult<Json<User>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let user = blog_service
        .get_user(&user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user))
}

async fn admin_get_user_by_email(
    State(state): State<AppState>,
    Path(email): Path<String>,
    _admin_user: AuthUser,
) -> AppResult<Json<User>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());
    let user = blog_service
        .get_user_by_email(&email)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user))
}

// Admin handlers using BlogPost model methods
async fn admin_create_post_with_model(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateBlogPostRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());

    // Extract fields from CreateBlogPostRequest for the model method
    let excerpt = payload.summary.clone().unwrap_or_else(|| {
        // Generate excerpt from body_html (first 150 chars)
        let plain_text = payload.body_html.chars().take(150).collect::<String>();
        format!("{}...", plain_text)
    });

    let blog_post = blog_service
        .create_post_with_model(
            payload.title,
            payload.body_html,
            excerpt,
            user.0.id.clone(),
            payload.tags,
            payload.visibility,
            payload.cover_image,
            vec![], // inline_images not in CreateBlogPostRequest
            payload.attachments,
        )
        .await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Blog post created successfully using model",
        "post": {
            "id": blog_post.id,
            "slug": blog_post.slug,
            "title": blog_post.title,
            "author_id": blog_post.author_id,
            "created_at": blog_post.created_at,
            "updated_at": blog_post.updated_at
        }
    })))
}

async fn admin_update_post_with_model(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    user: AuthUser,
    Json(payload): Json<CreateBlogPostRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let blog_service = BlogService::new(state.kv.clone(), state.db.clone());

    // Extract fields from CreateBlogPostRequest for the model method
    let excerpt = payload.summary.clone().unwrap_or_else(|| {
        // Generate excerpt from body_html (first 150 chars)
        let plain_text = payload.body_html.chars().take(150).collect::<String>();
        format!("{}...", plain_text)
    });

    let blog_post = blog_service
        .update_post_with_model(
            &slug,
            payload.title,
            payload.body_html,
            excerpt,
            payload.tags,
            payload.visibility,
            payload.cover_image,
            vec![], // inline_images not in CreateBlogPostRequest
            payload.attachments,
            user.0.id.clone(),
        )
        .await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Blog post updated successfully using model",
        "post": {
            "id": blog_post.id,
            "slug": blog_post.slug,
            "title": blog_post.title,
            "author_id": blog_post.author_id,
            "created_at": blog_post.created_at,
            "updated_at": blog_post.updated_at
        }
    })))
}

// Admin role checking handler
async fn admin_check_user_role(
    State(state): State<AppState>,
    Path((user_id, role_str)): Path<(String, String)>,
    Extension(_admin_user): Extension<UserResponse>,
) -> AppResult<Json<serde_json::Value>> {
    let auth_service = AuthService::new(state.db.clone(), state.config.clone());

    // Parse the role string using the auth service
    let required_role = auth_service.parse_user_role(&role_str)?;

    // Check if user has the specified role
    let has_role = auth_service.user_has_role(&user_id, required_role).await?;

    Ok(Json(serde_json::json!({
        "user_id": user_id,
        "role": role_str,
        "has_role": has_role
    })))
}

// Admin media upload handlers
async fn admin_upload_image(
    State(state): State<AppState>,
    Extension(_admin_user): Extension<UserResponse>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    // Extract upload data from payload
    let filename = payload
        .get("filename")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("filename is required".to_string()))?;

    let content_type = payload
        .get("content_type")
        .and_then(|v| v.as_str())
        .unwrap_or("image/jpeg");

    let file_data_base64 = payload
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("file data is required".to_string()))?;

    // Decode base64 data using the new Engine API
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let file_data = STANDARD
        .decode(file_data_base64)
        .map_err(|_| AppError::Validation("Invalid base64 file data".to_string()))?;

    // Use MediaUploader to upload image
    let uploader = MediaUploader::new(state.config.clone());
    let result = uploader
        .upload_image(file_data, filename, content_type)
        .await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Image uploaded successfully",
        "result": result
    })))
}

async fn admin_upload_file(
    State(state): State<AppState>,
    Extension(_admin_user): Extension<UserResponse>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    // Extract upload data from payload
    let filename = payload
        .get("filename")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("filename is required".to_string()))?;

    let content_type = payload
        .get("content_type")
        .and_then(|v| v.as_str())
        .unwrap_or("application/octet-stream");

    let file_data_base64 = payload
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("file data is required".to_string()))?;

    // Decode base64 data using the new Engine API
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let file_data = STANDARD
        .decode(file_data_base64)
        .map_err(|_| AppError::Validation("Invalid base64 file data".to_string()))?;

    // Use MediaUploader to upload file
    let uploader = MediaUploader::new(state.config.clone());
    let result = uploader
        .upload_file(file_data, filename, content_type)
        .await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "File uploaded successfully",
        "result": result
    })))
}

// Admin multipart upload handler using MediaUploader.process_multipart_upload
async fn admin_upload_multipart(
    State(state): State<AppState>,
    Extension(_admin_user): Extension<UserResponse>,
    multipart: axum::extract::Multipart,
) -> AppResult<Json<serde_json::Value>> {
    // Use MediaUploader to process multipart upload
    let uploader = MediaUploader::new(state.config.clone());
    let results = uploader.process_multipart_upload(multipart).await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Files uploaded successfully",
        "results": results,
        "count": results.len()
    })))
}
