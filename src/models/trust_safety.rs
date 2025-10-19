
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum TrustTrend {
    Improving,
    Stable,
    Declining,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrustComponents {
    pub reliability: f64,      // Completes projects on time
    pub communication: f64,    // Response time, clarity
    pub quality: f64,          // Work quality consistency
    pub professionalism: f64,  // Behavior, ethics
    pub transparency: f64,     // Honest timelines, budgets
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrustScore {
    pub overall_score: f64,  // 0-100
    pub components: TrustComponents,
    pub trend: TrustTrend,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserTrustScore {
    pub id: i32,
    pub user_id: Uuid,
    pub overall_score: f64,
    pub reliability: f64,
    pub communication: f64,
    pub quality: f64,
    pub professionalism: f64,
    pub transparency: f64,
    pub trend: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum FraudRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FraudDetectionResult {
    pub user_id: Uuid,
    pub risk_level: FraudRiskLevel,
    pub risk_score: f64,  // 0-100
    pub flags: Vec<FraudFlag>,
    pub recommended_actions: Vec<String>,
    pub requires_manual_review: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FraudFlag {
    pub flag_type: FraudFlagType,
    pub severity: f64,
    pub description: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FraudFlagType {
    UnusualBehavior,
    FakeProfile,
    PaymentFraud,
    ReviewManipulation,
    BotAccount,
    MultipleAccounts,
    SuspiciousActivity,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FraudAlert {
    pub id: i32,
    pub user_id: Uuid,
    pub flag_type: String,
    pub risk_score: f64,
    pub description: String,
    pub evidence: serde_json::Value,
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisputeRiskAlert {
    pub contract_id: Uuid,
    pub risk_level: f64,  // 0-100
    pub warning_signs: Vec<String>,
    pub suggested_actions: Vec<String>,
    pub mediation_recommended: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DisputeWarningType {
    CommunicationBreakdown,
    MissedMilestones,
    BudgetDisagreements,
    ScopeCreep,
    QualityConcerns,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DisputeRiskAssessment {
    pub id: i32,
    pub contract_id: Uuid,
    pub risk_score: f64,
    pub warning_signs: serde_json::Value,
    pub suggested_actions: serde_json::Value,
    pub mediation_recommended: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BehavioralAnalysis {
    pub user_id: Uuid,
    pub message_response_time: f64,  // Average in hours
    pub project_completion_rate: f64,  // 0-1
    pub budget_adherence_score: f64,  // 0-100
    pub timeline_accuracy_score: f64,  // 0-100
    pub client_satisfaction_avg: f64,  // 0-5
    pub quality_consistency: f64,  // 0-100
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustScoreRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FraudCheckRequest {
    pub user_id: Uuid,
    pub check_profile: bool,
    pub check_behavior: bool,
    pub check_reviews: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisputeRiskRequest {
    pub contract_id: Uuid,
}
