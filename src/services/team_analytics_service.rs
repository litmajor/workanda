
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::error::ApiError;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamAnalytics {
    pub team_id: Uuid,
    pub performance_score: f64,
    pub total_projects: i64,
    pub completed_projects: i64,
    pub success_rate: f64,
    pub average_rating: f64,
    pub total_revenue: Decimal,
    pub on_time_delivery: f64,
    pub member_retention: f64,
    pub client_satisfaction: f64,
    pub productivity_metrics: ProductivityMetrics,
    pub skill_distribution: Vec<SkillMetric>,
    pub revenue_trend: Vec<RevenueTrend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductivityMetrics {
    pub average_project_duration: i32,
    pub projects_per_month: f64,
    pub billable_hours: f64,
    pub utilization_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillMetric {
    pub skill: String,
    pub count: i64,
    pub proficiency_avg: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevenueTrend {
    pub month: String,
    pub revenue: Decimal,
    pub projects: i64,
}

pub struct TeamAnalyticsService {
    pool: PgPool,
}

impl TeamAnalyticsService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_team_analytics(
        &self,
        team_id: Uuid,
    ) -> Result<TeamAnalytics, ApiError> {
        let total_projects = self.get_total_projects(team_id).await?;
        let completed_projects = self.get_completed_projects(team_id).await?;
        let success_rate = if total_projects > 0 {
            (completed_projects as f64 / total_projects as f64) * 100.0
        } else {
            0.0
        };

        let average_rating = self.get_average_rating(team_id).await?;
        let total_revenue = self.get_total_revenue(team_id).await?;
        let on_time_delivery = self.get_on_time_delivery_rate(team_id).await?;
        let member_retention = self.get_member_retention_rate(team_id).await?;
        let client_satisfaction = self.get_client_satisfaction(team_id).await?;
        let productivity_metrics = self.get_productivity_metrics(team_id).await?;
        let skill_distribution = self.get_skill_distribution(team_id).await?;
        let revenue_trend = self.get_revenue_trend(team_id).await?;

        let performance_score = self.calculate_performance_score(
            success_rate,
            average_rating,
            on_time_delivery,
            member_retention,
            client_satisfaction,
        );

        Ok(TeamAnalytics {
            team_id,
            performance_score,
            total_projects,
            completed_projects,
            success_rate,
            average_rating,
            total_revenue,
            on_time_delivery,
            member_retention,
            client_satisfaction,
            productivity_metrics,
            skill_distribution,
            revenue_trend,
        })
    }

    async fn get_total_projects(&self, team_id: Uuid) -> Result<i64, ApiError> {
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM team_proposals WHERE team_id = $1 AND status = 'accepted'",
            team_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(0);

        Ok(count)
    }

    async fn get_completed_projects(&self, team_id: Uuid) -> Result<i64, ApiError> {
        let count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM contracts c
            JOIN team_proposals tp ON tp.job_id = c.id
            WHERE tp.team_id = $1 AND c.status = 'Completed'
            "#,
            team_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(0);

        Ok(count)
    }

    async fn get_average_rating(&self, team_id: Uuid) -> Result<f64, ApiError> {
        let avg: Option<f64> = sqlx::query_scalar!(
            r#"
            SELECT AVG(r.rating) FROM reviews r
            JOIN team_proposals tp ON tp.id = r.contract_id
            WHERE tp.team_id = $1
            "#,
            team_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(avg.unwrap_or(0.0))
    }

    async fn get_total_revenue(&self, team_id: Uuid) -> Result<Decimal, ApiError> {
        let revenue: Option<Decimal> = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(rd.total_amount), 0) as total
            FROM revenue_distributions rd
            WHERE rd.team_id = $1 AND rd.status = 'completed'
            "#,
            team_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(revenue.unwrap_or(Decimal::ZERO))
    }

    async fn get_on_time_delivery_rate(&self, team_id: Uuid) -> Result<f64, ApiError> {
        let total: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM milestones m JOIN team_proposals tp ON m.project_id = tp.id WHERE tp.team_id = $1",
            team_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(0);

        if total == 0 {
            return Ok(0.0);
        }

        let on_time: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM milestones m JOIN team_proposals tp ON m.project_id = tp.id WHERE tp.team_id = $1 AND m.completion_date <= m.due_date",
            team_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(0);

        Ok((on_time as f64 / total as f64) * 100.0)
    }

    async fn get_member_retention_rate(&self, team_id: Uuid) -> Result<f64, ApiError> {
        // Mock implementation - calculate based on team member history
        Ok(85.0)
    }

    async fn get_client_satisfaction(&self, team_id: Uuid) -> Result<f64, ApiError> {
        let avg_rating = self.get_average_rating(team_id).await?;
        Ok((avg_rating / 5.0) * 100.0)
    }

    async fn get_productivity_metrics(&self, team_id: Uuid) -> Result<ProductivityMetrics, ApiError> {
        Ok(ProductivityMetrics {
            average_project_duration: 45,
            projects_per_month: 2.5,
            billable_hours: 160.0,
            utilization_rate: 85.0,
        })
    }

    async fn get_skill_distribution(&self, team_id: Uuid) -> Result<Vec<SkillMetric>, ApiError> {
        let skills = sqlx::query!(
            r#"
            SELECT skill, COUNT(*) as count
            FROM team_members tm
            JOIN freelancer_skills fs ON tm.freelancer_id = fs.freelancer_id
            WHERE tm.team_id = $1
            GROUP BY skill
            "#,
            team_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(skills.into_iter().map(|s| SkillMetric {
            skill: s.skill.unwrap_or_default(),
            count: s.count.unwrap_or(0),
            proficiency_avg: 4.2,
        }).collect())
    }

    async fn get_revenue_trend(&self, team_id: Uuid) -> Result<Vec<RevenueTrend>, ApiError> {
        let trends = sqlx::query!(
            r#"
            SELECT 
                TO_CHAR(rd.created_at, 'YYYY-MM') as month,
                SUM(rd.total_amount) as revenue,
                COUNT(*) as projects
            FROM revenue_distributions rd
            WHERE rd.team_id = $1 AND rd.status = 'completed'
            GROUP BY TO_CHAR(rd.created_at, 'YYYY-MM')
            ORDER BY month DESC
            LIMIT 12
            "#,
            team_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(trends.into_iter().map(|t| RevenueTrend {
            month: t.month.unwrap_or_default(),
            revenue: t.revenue.unwrap_or(Decimal::ZERO),
            projects: t.projects.unwrap_or(0),
        }).collect())
    }

    fn calculate_performance_score(
        &self,
        success_rate: f64,
        average_rating: f64,
        on_time_delivery: f64,
        member_retention: f64,
        client_satisfaction: f64,
    ) -> f64 {
        (success_rate * 0.25 + 
         (average_rating / 5.0 * 100.0) * 0.25 +
         on_time_delivery * 0.25 +
         member_retention * 0.125 +
         client_satisfaction * 0.125)
    }
}
