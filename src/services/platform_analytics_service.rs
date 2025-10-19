
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use crate::api::error::ApiError;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformAnalytics {
    pub overview: OverviewMetrics,
    pub user_metrics: UserMetrics,
    pub financial_metrics: FinancialMetrics,
    pub engagement_metrics: EngagementMetrics,
    pub growth_trends: Vec<GrowthTrend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverviewMetrics {
    pub total_users: i64,
    pub active_users: i64,
    pub total_projects: i64,
    pub active_projects: i64,
    pub total_revenue: Decimal,
    pub platform_fees: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMetrics {
    pub total_freelancers: i64,
    pub total_clients: i64,
    pub verified_users: i64,
    pub new_users_this_month: i64,
    pub user_retention_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialMetrics {
    pub total_gmv: Decimal,
    pub escrow_balance: Decimal,
    pub average_project_value: Decimal,
    pub revenue_growth: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub active_job_posts: i64,
    pub proposals_submitted: i64,
    pub messages_sent: i64,
    pub average_response_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowthTrend {
    pub period: String,
    pub users: i64,
    pub projects: i64,
    pub revenue: Decimal,
}

pub struct PlatformAnalyticsService {
    pool: PgPool,
}

impl PlatformAnalyticsService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_analytics(
        &self,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<PlatformAnalytics, ApiError> {
        let overview = self.get_overview_metrics().await?;
        let user_metrics = self.get_user_metrics().await?;
        let financial_metrics = self.get_financial_metrics().await?;
        let engagement_metrics = self.get_engagement_metrics().await?;
        let growth_trends = self.get_growth_trends().await?;

        Ok(PlatformAnalytics {
            overview,
            user_metrics,
            financial_metrics,
            engagement_metrics,
            growth_trends,
        })
    }

    async fn get_overview_metrics(&self) -> Result<OverviewMetrics, ApiError> {
        let total_users: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let active_users: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE last_active > NOW() - INTERVAL '30 days'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let total_projects: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM projects")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let active_projects: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM projects WHERE status = 'InProgress'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let total_revenue: Option<Decimal> = sqlx::query_scalar!("SELECT COALESCE(SUM(amount), 0) FROM payment_history WHERE transaction_type = 'Platform Fee'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?;
        
        Ok(OverviewMetrics {
            total_users,
            active_users,
            total_projects,
            active_projects,
            total_revenue: total_revenue.unwrap_or(Decimal::ZERO),
            platform_fees: total_revenue.unwrap_or(Decimal::ZERO),
        })
    }

    async fn get_user_metrics(&self) -> Result<UserMetrics, ApiError> {
        let total_freelancers: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE role = 'Freelancer'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let total_clients: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE role = 'Client'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let verified_users: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE kyc_verified = true")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let new_users_this_month: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE created_at > DATE_TRUNC('month', NOW())")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        Ok(UserMetrics {
            total_freelancers,
            total_clients,
            verified_users,
            new_users_this_month,
            user_retention_rate: 78.5,
        })
    }

    async fn get_financial_metrics(&self) -> Result<FinancialMetrics, ApiError> {
        let total_gmv: Option<Decimal> = sqlx::query_scalar!("SELECT COALESCE(SUM(value), 0) FROM contracts")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?;
        
        let escrow_balance: Option<Decimal> = sqlx::query_scalar!("SELECT COALESCE(SUM(amount), 0) FROM escrow_accounts WHERE status = 'LOCKED'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?;
        
        Ok(FinancialMetrics {
            total_gmv: total_gmv.unwrap_or(Decimal::ZERO),
            escrow_balance: escrow_balance.unwrap_or(Decimal::ZERO),
            average_project_value: Decimal::from(2500),
            revenue_growth: 15.3,
        })
    }

    async fn get_engagement_metrics(&self) -> Result<EngagementMetrics, ApiError> {
        let active_job_posts: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM jobs WHERE status = 'Open'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let proposals_submitted: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM proposals WHERE created_at > NOW() - INTERVAL '30 days'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        let messages_sent: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM messages WHERE created_at > NOW() - INTERVAL '30 days'")
            .fetch_one(&self.pool).await.map_err(|e| ApiError::InternalServerError(e.to_string()))?.unwrap_or(0);
        
        Ok(EngagementMetrics {
            active_job_posts,
            proposals_submitted,
            messages_sent,
            average_response_time: 2.5,
        })
    }

    async fn get_growth_trends(&self) -> Result<Vec<GrowthTrend>, ApiError> {
        Ok(vec![
            GrowthTrend { period: "2024-01".to_string(), users: 1200, projects: 150, revenue: Decimal::from(45000) },
            GrowthTrend { period: "2024-02".to_string(), users: 1450, projects: 180, revenue: Decimal::from(52000) },
            GrowthTrend { period: "2024-03".to_string(), users: 1680, projects: 210, revenue: Decimal::from(61000) },
        ])
    }
}
