
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;
use std::collections::HashMap;

// Project Success Prediction Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskFactor {
    BudgetTooLow { recommended: f64 },
    TimelineTooTight { recommended: i32 },
    SkillGap { missing_skills: Vec<String> },
    CommunicationMismatch,
    ComplexityMismatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRiskAssessment {
    pub risk_level: RiskLevel,
    pub success_probability: f64,
    pub risk_factors: Vec<RiskFactor>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSuccessPredictionRequest {
    pub project_id: i32,
    pub freelancer_id: Uuid,
    pub budget: f64,
    pub timeline_days: i32,
    pub required_skills: Vec<String>,
    pub complexity_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreelancerTrackRecord {
    pub freelancer_id: Uuid,
    pub similar_projects_completed: i32,
    pub success_rate: f64,
    pub avg_completion_time: f64,
    pub avg_budget_adherence: f64,
    pub communication_score: f64,
}

// Dynamic Pricing Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingSuggestion {
    pub suggested_range: (f64, f64),
    pub market_average: f64,
    pub competitive_rate: f64,
    pub confidence_level: f64,
    pub factors: Vec<PricingFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingFactor {
    pub factor_name: String,
    pub impact: f64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingRequest {
    pub project_type: String,
    pub required_skills: Vec<String>,
    pub complexity_level: f32,
    pub estimated_hours: i32,
    pub freelancer_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketPricingData {
    pub id: i32,
    pub project_type: String,
    pub skill_category: String,
    pub avg_hourly_rate: f64,
    pub min_rate: f64,
    pub max_rate: f64,
    pub sample_size: i32,
    pub last_updated: NaiveDateTime,
}

// Timeline Estimation Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEstimation {
    pub estimated_days: i32,
    pub confidence_level: f64,
    pub breakdown: Vec<TaskEstimate>,
    pub risk_buffer_days: i32,
    pub factors: Vec<TimelineFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEstimate {
    pub task_name: String,
    pub estimated_hours: f64,
    pub complexity: f32,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineFactor {
    pub factor_name: String,
    pub impact_days: i32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineRequest {
    pub project_type: String,
    pub required_skills: Vec<String>,
    pub complexity_level: f32,
    pub team_size: i32,
    pub freelancer_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HistoricalCompletion {
    pub id: i32,
    pub project_type: String,
    pub complexity_level: f32,
    pub team_size: i32,
    pub actual_days: i32,
    pub estimated_days: i32,
    pub success: bool,
    pub completed_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamVelocity {
    pub team_id: Option<i32>,
    pub avg_velocity: f64,
    pub consistency_score: f64,
    pub recent_performance: Vec<f64>,
}
