use crate::config::AppConfig;
use crate::error::{AppError, AppResult};
use crate::models::{
    Claims, GoogleAuthRequest, LoginRequest, LoginResponse, Revocation, User, UserResponse, UserRole,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct AuthService {
    pub db: SqlitePool,
    pub config: AppConfig,
}

impl AuthService {
    pub fn new(db: SqlitePool, config: AppConfig) -> Self {
        Self { db, config }
    }

    /// Create JWT token with revocation support
    pub async fn create_jwt_token(&self, user_id: &str) -> AppResult<String> {
        let jti = Uuid::new_v4().to_string();
        let now = Utc::now();
        let exp = now + chrono::Duration::days(7);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
            jti: jti.clone(),
            iat: now.timestamp() as usize,
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.config.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }

    /// Verify JWT token and check revocation list
    pub async fn verify_jwt_token(&self, token: &str) -> AppResult<Claims> {
        // Decode JWT token
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AppError::Auth("Invalid token".to_string()))?;

        // Check if token is revoked
        let revoked: Option<String> =
            sqlx::query_scalar("SELECT jti FROM revocations WHERE jti = $1")
                .bind(&token_data.claims.jti)
                .fetch_optional(&self.db)
                .await?;

        if revoked.is_some() {
            return Err(AppError::Auth("Token has been revoked".to_string()));
        }

        Ok(token_data.claims)
    }

    /// Revoke a JWT token
    pub async fn revoke_token(&self, token: &str) -> AppResult<()> {
        let claims = self.verify_jwt_token(token).await?;

        let revocation = Revocation::new(
            claims.jti,
            Some(claims.sub),
            Some(chrono::DateTime::from_timestamp(claims.exp as i64, 0).unwrap_or(Utc::now())),
        );

        sqlx::query("INSERT INTO revocations (jti, user_id, revoked_at, expires_at) VALUES ($1, $2, $3, $4)")
            .bind(&revocation.jti)
            .bind(&revocation.user_id)
            .bind(&revocation.revoked_at)
            .bind(&revocation.expires_at)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    /// Get user by email (for authentication)
    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, email, role, google_id, full_name, created_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await?;
        Ok(user)
    }

    /// Get user by Google ID
    pub async fn get_user_by_google_id(&self, google_id: &str) -> AppResult<Option<User>> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, email, role, google_id, full_name, created_at FROM users WHERE google_id = $1"
        )
        .bind(google_id)
        .fetch_optional(&self.db)
        .await?;
        Ok(user)
    }

    /// Create a new user with Google authentication
    pub async fn create_user_with_google(
        &self,
        email: String,
        google_id: String,
        full_name: Option<String>,
    ) -> AppResult<User> {
        let user = User::new_with_google(email, UserRole::Student, google_id, full_name);

        sqlx::query("INSERT INTO users (id, email, role, google_id, full_name, created_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&user.id)
            .bind(&user.email)
            .bind(&user.role)
            .bind(&user.google_id)
            .bind(&user.full_name)
            .bind(&user.created_at)
            .execute(&self.db)
            .await?;

        Ok(user)
    }

    /// Verify session cookie and return user info
    pub async fn verify_session_cookie(&self, cookie_value: &str) -> AppResult<UserResponse> {
        let claims = self.verify_jwt_token(cookie_value).await?;

        // Get user details
        let user: Option<User> = sqlx::query_as(
            "SELECT id, email, role, google_id, full_name, created_at FROM users WHERE id = $1",
        )
        .bind(&claims.sub)
        .fetch_optional(&self.db)
        .await?;

        let user = user.ok_or_else(|| AppError::Auth("User not found".to_string()))?;

        // Validate that the user role is valid using the get_role method
        if user.get_role().is_none() {
            return Err(AppError::Auth("Invalid user role".to_string()));
        }

        Ok(UserResponse {
            id: user.id,
            email: user.email,
            role: user.role,
            full_name: user.full_name,
        })
    }
    /// Check if user has specific role
    pub async fn user_has_role(&self, user_id: &str, required_role: UserRole) -> AppResult<bool> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, email, role, google_id, full_name, created_at FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?;

        if let Some(user) = user {
            if let Some(user_role) = user.get_role() {
                return Ok(user_role == required_role);
            }
        }
        
        Ok(false)
    }

    /// Parse role from string and validate
    pub fn parse_user_role(&self, role_str: &str) -> AppResult<UserRole> {
        UserRole::from_str(role_str)
            .ok_or_else(|| AppError::Auth(format!("Invalid user role: {}", role_str)))
    }

    /// Create cookie string for JWT token
    pub fn create_cookie_string(&self, token: &str) -> String {
        if self.config.environment == "development" {
            format!(
                "session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
                token,
                7 * 24 * 60 * 60 // 7 days in seconds
            )
        } else {
            format!(
                "session={}; Domain=.llacademy.ng; Path=/; Secure; HttpOnly; SameSite=None; Max-Age={}",
                token,
                7 * 24 * 60 * 60 // 7 days in seconds
            )
        }
    }

    /// Create cookie string for logout (clears the cookie)
    pub fn create_logout_cookie_string(&self) -> String {
        if self.config.environment == "development" {
            "session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0".to_string()
        } else {
            "session=; Domain=.llacademy.ng; Path=/; Secure; HttpOnly; SameSite=None; Max-Age=0"
                .to_string()
        }
    }

    /// Logout by revoking the token
    pub async fn logout_with_cookie(&self, cookie_value: &str) -> AppResult<String> {
        // Revoke the token
        if let Err(_) = self.revoke_token(cookie_value).await {
            // If token is already invalid, that's fine for logout
        }

        // Return cookie clearing string
        Ok(self.create_logout_cookie_string())
    }

    /// Google OAuth login implementation
    pub async fn google_oauth_login(&self, payload: GoogleAuthRequest) -> AppResult<(LoginResponse, String)> {
        // Validate the authorization code is present
        if payload.code.is_empty() {
            return Err(AppError::Auth("Authorization code is required".to_string()));
        }
        
        // Validate state parameter if provided (CSRF protection)
        if let Some(state) = &payload.state {
            if state.is_empty() {
                return Err(AppError::Auth("Invalid state parameter".to_string()));
            }
            // In production, verify state matches what was sent to Google
            tracing::info!("OAuth state parameter received: {}", state);
        }
        
        // Log the OAuth attempt for audit purposes
        tracing::info!("Processing Google OAuth login with code: {}", &payload.code[..8.min(payload.code.len())]);
        
        // TODO: Implement actual Google OAuth flow
        // For now, simulate the flow to demonstrate the architecture
        // In a real implementation, you would:
        // 1. Verify the authorization code with Google OAuth API
        // 2. Exchange the code for an access token using payload.code
        // 3. Get user info from Google using the access token
        
        // Simulated Google user info (in real implementation, this comes from Google API)
        let google_id = format!("google_user_{}", &payload.code[..8.min(payload.code.len())]);
        let email = "user@example.com";
        let full_name = Some("Test User".to_string());
        
        // Check if user exists by Google ID
        let user = self.get_user_by_google_id(&google_id).await?;
        
        let user = if let Some(user) = user {
            user
        } else {
            // Check if user exists by email
            if let Some(existing_user) = self.get_user_by_email(email).await? {
                // Update existing user with Google ID
                sqlx::query("UPDATE users SET google_id = ? WHERE id = ?")
                    .bind(google_id)
                    .bind(&existing_user.id)
                    .execute(&self.db)
                    .await?;
                existing_user
            } else {
                // Create new user with Google auth
                self.create_user_with_google(
                    email.to_string(),
                    google_id.to_string(),
                    full_name,
                ).await?
            }
        };
        
        // Create JWT token
        let token = self.create_jwt_token(&user.id).await?;
        let cookie = self.create_cookie_string(&token);
        
        let response = LoginResponse {
            token: token.clone(),
            user: UserResponse {
                id: user.id,
                email: user.email,
                role: user.role,
                full_name: user.full_name,
            },
            expires_at: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
        };
        
        Ok((response, cookie))
    }

    /// Passwordless login method - redirects to Google OAuth
    pub async fn login(&self, payload: LoginRequest) -> AppResult<(LoginResponse, String)> {
        // Log the login attempt for audit purposes
        tracing::info!("Passwordless login attempt for email: {}", payload.email);
        
        // Validate input fields are not empty
        if payload.email.trim().is_empty() {
            return Err(AppError::Auth("Email is required".to_string()));
        }
        
        // In the passwordless system, all authentication should be done via Google OAuth
        // Return error with instructions to use Google OAuth
        Err(AppError::Auth(
            "Password-based login is deprecated. Please use Google OAuth for authentication.".to_string(),
        ))
    }

    /// Verify session token (alias for verify_session_cookie for backward compatibility)
    pub async fn verify_session_token(&self, token: &str) -> AppResult<UserResponse> {
        self.verify_session_cookie(token).await
    }
}
