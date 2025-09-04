use crate::auth::AuthService;
use crate::models::UserResponse;
use crate::AppState;
use axum::{
    extract::{Extension, Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

// Extension type for authenticated user
pub type AuthUser = Extension<UserResponse>;

// Middleware to verify authentication for protected routes
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_service = AuthService::new(state.db.clone(), state.config.clone());

    let headers = request.headers();

    // First try Bearer token (for API compatibility)
    if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                match auth_service.verify_session_token(token).await {
                    Ok(user) => {
                        // Add user to request extensions
                        request.extensions_mut().insert(user);
                        return Ok(next.run(request).await);
                    }
                    Err(_) => return Err(StatusCode::UNAUTHORIZED),
                }
            }
        }
    }

    // Then try session cookie
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("session=") {
                    let session_value = &cookie[8..];
                    match auth_service.verify_session_cookie(session_value).await {
                        Ok(user) => {
                            // Add user to request extensions
                            request.extensions_mut().insert(user);
                            return Ok(next.run(request).await);
                        }
                        Err(_) => return Err(StatusCode::UNAUTHORIZED),
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Middleware to verify admin role for admin routes
pub async fn admin_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    // Get user from request extensions (set by auth_middleware)
    if let Some(user) = request.extensions().get::<UserResponse>() {
        if user.role == "admin" {
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::FORBIDDEN)
}
