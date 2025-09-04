use crate::error::AppResult;
use crate::models::{AuditAction, AuditLog};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use serde_json::Value;
use sqlx::SqlitePool;

/// Service for managing incremental JSON audit logging with monthly sharding
pub struct AuditService {
    pub db: SqlitePool,
}

impl AuditService {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    /// Log an audit action using incremental JSON append strategy
    pub async fn log_action(
        &self,
        user_id: &str,
        action: String,
        resource_id: Option<String>,
        details: Option<Value>,
    ) -> AppResult<()> {
        let session_date = Utc::now().format("%Y-%m-%d").to_string();
        let audit_action = AuditAction::new(action, resource_id, details);

        // Get current month table name
        let table_name = self.get_current_audit_table().await?;

        // Try to find existing audit log for this user and date
        let existing_log: Option<AuditLog> = sqlx::query_as(&format!(
            "SELECT id, user_id, session_date, actions, created_at, updated_at FROM {} WHERE user_id = $1 AND session_date = $2 ORDER BY updated_at DESC LIMIT 1",
            table_name
        ))
        .bind(user_id)
        .bind(&session_date)
        .fetch_optional(&self.db)
        .await?;

        match existing_log {
            Some(log) => {
                // Parse existing actions
                let mut actions: Vec<AuditAction> =
                    serde_json::from_str(&log.actions).unwrap_or_else(|_| Vec::new());

                // Check if we need to rotate (create new row)
                if self.should_rotate_row(&actions, &audit_action)? {
                    // Create new row
                    self.create_new_audit_row(user_id, &session_date, audit_action, &table_name)
                        .await?;
                } else {
                    // Append to existing row
                    actions.push(audit_action);
                    let updated_actions = serde_json::to_string(&actions)?;

                    sqlx::query(&format!(
                        "UPDATE {} SET actions = $1, updated_at = $2 WHERE id = $3",
                        table_name
                    ))
                    .bind(&updated_actions)
                    .bind(Utc::now())
                    .bind(&log.id)
                    .execute(&self.db)
                    .await?;
                }
            }
            None => {
                // Create new audit log entry
                self.create_new_audit_row(user_id, &session_date, audit_action, &table_name)
                    .await?;
            }
        }

        Ok(())
    }

    /// Create a new audit log row
    async fn create_new_audit_row(
        &self,
        user_id: &str,
        session_date: &str,
        audit_action: AuditAction,
        table_name: &str,
    ) -> AppResult<()> {
        let actions = vec![audit_action];
        let actions_json = serde_json::to_string(&actions)?;

        let audit_log = AuditLog::new(user_id.to_string(), session_date.to_string());

        sqlx::query(&format!(
            "INSERT INTO {} (id, user_id, session_date, actions, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)",
            table_name
        ))
        .bind(&audit_log.id)
        .bind(&audit_log.user_id)
        .bind(&audit_log.session_date)
        .bind(&actions_json)
        .bind(&audit_log.created_at)
        .bind(&audit_log.updated_at)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    /// Check if we should rotate to a new row (based on size or action count)
    fn should_rotate_row(
        &self,
        actions: &[AuditAction],
        new_action: &AuditAction,
    ) -> AppResult<bool> {
        // Rotate if we have 50+ actions
        if actions.len() >= 50 {
            return Ok(true);
        }

        // Rotate if JSON would exceed 1MB
        let mut test_actions = actions.to_vec();
        test_actions.push(new_action.clone());
        let test_json = serde_json::to_string(&test_actions)?;

        if test_json.len() > 1_048_576 {
            // 1MB
            return Ok(true);
        }

        Ok(false)
    }

    /// Get the current month's audit table name and ensure it exists
    async fn get_current_audit_table(&self) -> AppResult<String> {
        let now = Utc::now();
        let table_name = format!("audit_logs_{}_{:02}", now.year(), now.month());

        // Create table if it doesn't exist
        self.ensure_audit_table_exists(&table_name).await?;

        Ok(table_name)
    }

    /// Ensure the audit table for the given month exists
    async fn ensure_audit_table_exists(&self, table_name: &str) -> AppResult<()> {
        let create_table_sql = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                session_date TEXT NOT NULL,
                actions TEXT NOT NULL DEFAULT '[]',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )
            "#,
            table_name
        );

        sqlx::query(&create_table_sql).execute(&self.db).await?;

        // Create indexes
        let index_sql = format!(
            r#"
            CREATE INDEX IF NOT EXISTS idx_{}_user_id ON {}(user_id);
            CREATE INDEX IF NOT EXISTS idx_{}_session_date ON {}(session_date);
            CREATE INDEX IF NOT EXISTS idx_{}_created_at ON {}(created_at);
            "#,
            table_name, table_name, table_name, table_name, table_name, table_name
        );

        sqlx::query(&index_sql).execute(&self.db).await?;

        Ok(())
    }

    /// Get audit logs for a user within a date range
    pub async fn get_user_audit_logs(
        &self,
        user_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> AppResult<Vec<AuditAction>> {
        let mut all_actions = Vec::new();

        // Generate table names for the date range
        let table_names = self.get_table_names_for_range(start_date, end_date);

        for table_name in table_names {
            // Check if table exists
            let table_exists: bool = sqlx::query_scalar(
                "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name = $1",
            )
            .bind(&table_name)
            .fetch_one(&self.db)
            .await?;

            if !table_exists {
                continue;
            }

            let logs: Vec<AuditLog> = sqlx::query_as(&format!(
                "SELECT id, user_id, session_date, actions, created_at, updated_at FROM {} WHERE user_id = $1 AND created_at BETWEEN $2 AND $3 ORDER BY created_at",
                table_name
            ))
            .bind(user_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&self.db)
            .await?;

            for log in logs {
                let actions: Vec<AuditAction> =
                    serde_json::from_str(&log.actions).unwrap_or_else(|_| Vec::new());

                // Filter actions by date range
                for action in actions {
                    if action.timestamp >= start_date && action.timestamp <= end_date {
                        all_actions.push(action);
                    }
                }
            }
        }

        // Sort by timestamp
        all_actions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(all_actions)
    }

    /// Generate table names for a date range
    fn get_table_names_for_range(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Vec<String> {
        let mut table_names = Vec::new();
        let mut current = start_date;

        while current <= end_date {
            let table_name = format!("audit_logs_{}_{:02}", current.year(), current.month());
            if !table_names.contains(&table_name) {
                table_names.push(table_name);
            }

            // Move to next month
            if current.month() == 12 {
                current = current
                    .with_year(current.year() + 1)
                    .unwrap()
                    .with_month(1)
                    .unwrap();
            } else {
                current = current.with_month(current.month() + 1).unwrap();
            }
        }

        table_names
    }

    /// Clean up old audit tables (keep last 3 months)
    pub async fn cleanup_old_audit_tables(&self) -> AppResult<()> {
        let now = Utc::now();
        let cutoff_date = now - chrono::Duration::days(90); // 3 months ago

        // Get all audit table names
        let table_names: Vec<String> = sqlx::query_scalar(
            "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'audit_logs_%'",
        )
        .fetch_all(&self.db)
        .await?;

        for table_name in table_names {
            if let Some(date) = self.parse_table_date(&table_name) {
                if date < cutoff_date {
                    // Export table data before dropping (could be implemented later)
                    tracing::info!("Dropping old audit table: {}", table_name);
                    sqlx::query(&format!("DROP TABLE IF EXISTS {}", table_name))
                        .execute(&self.db)
                        .await?;
                }
            }
        }

        Ok(())
    }

    /// Parse date from table name (audit_logs_YYYY_MM)
    fn parse_table_date(&self, table_name: &str) -> Option<DateTime<Utc>> {
        let parts: Vec<&str> = table_name.split('_').collect();
        if parts.len() >= 4 {
            if let (Ok(year), Ok(month)) = (parts[2].parse::<i32>(), parts[3].parse::<u32>()) {
                return Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).single();
            }
        }
        None
    }
}
