
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::api::error::ApiError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<i32>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAuditLogRequest {
    pub user_id: Option<i32>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<i32>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogFilter {
    pub user_id: Option<i32>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
}

pub struct AuditLogService {
    pool: PgPool,
}

impl AuditLogService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_log(
        &self,
        request: CreateAuditLogRequest,
    ) -> Result<AuditLog, ApiError> {
        let log = sqlx::query_as!(
            AuditLog,
            r#"
            INSERT INTO audit_logs (user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            RETURNING id, user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at
            "#,
            request.user_id,
            request.action,
            request.resource_type,
            request.resource_id,
            request.details,
            request.ip_address,
            request.user_agent
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(log)
    }

    pub async fn get_logs(
        &self,
        filter: AuditLogFilter,
    ) -> Result<Vec<AuditLog>, ApiError> {
        let mut query = String::from("SELECT * FROM audit_logs WHERE 1=1");
        
        if filter.user_id.is_some() {
            query.push_str(&format!(" AND user_id = {}", filter.user_id.unwrap()));
        }
        if let Some(action) = &filter.action {
            query.push_str(&format!(" AND action = '{}'", action));
        }
        if let Some(resource_type) = &filter.resource_type {
            query.push_str(&format!(" AND resource_type = '{}'", resource_type));
        }
        if filter.start_date.is_some() {
            query.push_str(&format!(" AND created_at >= '{}'", filter.start_date.unwrap()));
        }
        if filter.end_date.is_some() {
            query.push_str(&format!(" AND created_at <= '{}'", filter.end_date.unwrap()));
        }
        
        query.push_str(" ORDER BY created_at DESC");
        
        if let Some(limit) = filter.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        let logs = sqlx::query_as::<_, AuditLog>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(logs)
    }

    pub async fn log_user_action(
        &self,
        user_id: i32,
        action: &str,
        resource_type: &str,
        resource_id: Option<i32>,
        details: serde_json::Value,
    ) -> Result<(), ApiError> {
        self.create_log(CreateAuditLogRequest {
            user_id: Some(user_id),
            action: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id,
            details,
            ip_address: None,
            user_agent: None,
        }).await?;

        Ok(())
    }

    pub async fn get_user_activity(
        &self,
        user_id: i32,
        limit: i64,
    ) -> Result<Vec<AuditLog>, ApiError> {
        self.get_logs(AuditLogFilter {
            user_id: Some(user_id),
            action: None,
            resource_type: None,
            start_date: None,
            end_date: None,
            limit: Some(limit),
        }).await
    }
}
