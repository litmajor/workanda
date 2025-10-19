pub mod user;
pub mod job;
pub mod proposal;
pub mod contract;
pub mod milestone;
pub mod payment;
pub mod escrow;
pub mod review;
pub mod message;
pub mod team_proposal;
pub mod revenue_distribution;
pub mod ai_matching;
pub mod predictive_analytics;
pub mod proposal_assistant;
pub mod trust_safety;
pub use ai_matching::{
    DynamicTeamSuggestion, TeamMemberProfile, CommunicationStyle,
    CollaborationHistory, TeamDynamics, SkillSynergyAnalysis,
    SkillPair, SkillGap, SkillOverlap, DynamicTeamRequest,
};
pub use predictive_analytics::{
    ProjectRiskAssessment, RiskLevel, RiskFactor, PricingSuggestion,
    PricingFactor, TimelineEstimation, TaskEstimate, TimelineFactor,
};
pub use proposal_assistant::{
    ProposalAssistant, Section, ClientProfile, ProposalAnalysisRequest,
    ProposalAnalysisResponse, ProposalImprovements, JobCategorization,
    ProjectType, ComplexityLevel, SmartSearchRequest, SearchType,
    SearchFilters, SmartSearchResponse, SearchResult, IntentAnalysis,
    TrendingSkill,
};
pub use trust_safety::{
    TrustScore, TrustComponents, TrustTrend, UserTrustScore,
    FraudDetectionResult, FraudFlag, FraudFlagType, FraudRiskLevel,
    FraudAlert, DisputeRiskAlert, DisputeWarningType, DisputeRiskAssessment,
    BehavioralAnalysis, TrustScoreRequest, FraudCheckRequest, DisputeRiskRequest,
};

pub use self::user::*;
pub use self::job::*;
pub use self::proposal::*;
pub use self::contract::*;
pub use self::milestone::*;
pub use self::payment::*;
pub use self::escrow::*;
pub use self::review::*;
pub use self::message::*;
pub use self::team_proposal::*;
pub use self::revenue_distribution::*;
pub use self::ai_matching::*;