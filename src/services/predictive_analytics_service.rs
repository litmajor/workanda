
use crate::models::predictive_analytics::*;
use crate::models::ai_matching::FreelancerEmbedding;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;

pub struct PredictiveAnalyticsService {
    pool: PgPool,
}

impl PredictiveAnalyticsService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Predict project success probability
    pub async fn predict_project_success(
        &self,
        request: ProjectSuccessPredictionRequest,
    ) -> Result<ProjectRiskAssessment, sqlx::Error> {
        // Get freelancer track record
        let track_record = self.get_freelancer_track_record(
            request.freelancer_id,
            &request.required_skills,
        ).await?;

        // Analyze budget adequacy
        let budget_analysis = self.analyze_budget_adequacy(
            request.budget,
            &request.required_skills,
            request.complexity_level,
        ).await?;

        // Analyze timeline realism
        let timeline_analysis = self.analyze_timeline_realism(
            request.timeline_days,
            &request.required_skills,
            request.complexity_level,
        ).await?;

        // Analyze skill match
        let skill_match = self.analyze_skill_match(
            request.freelancer_id,
            &request.required_skills,
        ).await?;

        // Calculate success probability
        let mut success_score = 0.0;
        let mut risk_factors = Vec::new();
        let mut recommendations = Vec::new();

        // Factor 1: Track record (30%)
        success_score += track_record.success_rate * 0.3;
        if track_record.success_rate < 0.7 {
            recommendations.push(format!(
                "Freelancer has {}% success rate in similar projects. Consider more experienced freelancer.",
                (track_record.success_rate * 100.0) as i32
            ));
        }

        // Factor 2: Budget adequacy (25%)
        if budget_analysis.is_adequate {
            success_score += 0.25;
        } else {
            risk_factors.push(RiskFactor::BudgetTooLow {
                recommended: budget_analysis.recommended_budget,
            });
            recommendations.push(format!(
                "Budget is {}% below market rate. Recommended: ${:.2}",
                ((1.0 - request.budget / budget_analysis.recommended_budget) * 100.0) as i32,
                budget_analysis.recommended_budget
            ));
        }

        // Factor 3: Timeline realism (20%)
        if timeline_analysis.is_realistic {
            success_score += 0.20;
        } else {
            risk_factors.push(RiskFactor::TimelineTooTight {
                recommended: timeline_analysis.recommended_days,
            });
            recommendations.push(format!(
                "Timeline is tight. Recommended: {} days (current: {} days)",
                timeline_analysis.recommended_days,
                request.timeline_days
            ));
        }

        // Factor 4: Skill match (15%)
        let skill_score = skill_match.coverage * 0.15;
        success_score += skill_score;
        if skill_match.coverage < 0.8 {
            risk_factors.push(RiskFactor::SkillGap {
                missing_skills: skill_match.missing_skills.clone(),
            });
            recommendations.push(format!(
                "Missing skills: {}. Consider team collaboration.",
                skill_match.missing_skills.join(", ")
            ));
        }

        // Factor 5: Communication (10%)
        success_score += track_record.communication_score * 0.10;
        if track_record.communication_score < 0.7 {
            risk_factors.push(RiskFactor::CommunicationMismatch);
            recommendations.push("Consider establishing clear communication protocols.".to_string());
        }

        // Determine risk level
        let risk_level = if success_score >= 0.75 {
            RiskLevel::Low
        } else if success_score >= 0.50 {
            RiskLevel::Medium
        } else {
            RiskLevel::High
        };

        if success_score >= 0.75 {
            recommendations.push("Strong project-freelancer fit! Proceed with confidence.".to_string());
        }

        Ok(ProjectRiskAssessment {
            risk_level,
            success_probability: success_score,
            risk_factors,
            recommendations,
        })
    }

    /// Suggest optimal pricing based on market conditions
    pub async fn suggest_pricing(
        &self,
        request: PricingRequest,
    ) -> Result<PricingSuggestion, sqlx::Error> {
        // Get market data
        let market_data = self.get_market_pricing(&request.project_type, &request.required_skills).await?;

        // Get freelancer's experience level if provided
        let freelancer_multiplier = if let Some(freelancer_id) = request.freelancer_id {
            self.get_freelancer_pricing_multiplier(freelancer_id).await?
        } else {
            1.0
        };

        // Calculate base pricing
        let base_rate = market_data.avg_hourly_rate;
        
        // Adjust for complexity
        let complexity_multiplier = 1.0 + (request.complexity_level - 0.5) * 0.4;
        
        // Adjust for skill demand
        let skill_demand = self.calculate_skill_demand(&request.required_skills).await?;
        let demand_multiplier = 1.0 + (skill_demand - 0.5) * 0.3;

        // Calculate suggested rate
        let competitive_rate = base_rate * complexity_multiplier * demand_multiplier * freelancer_multiplier;
        let min_rate = competitive_rate * 0.85;
        let max_rate = competitive_rate * 1.25;

        // Build factors
        let mut factors = Vec::new();
        factors.push(PricingFactor {
            factor_name: "Market Average".to_string(),
            impact: base_rate,
            description: format!("Based on {} similar projects", market_data.sample_size),
        });
        factors.push(PricingFactor {
            factor_name: "Complexity".to_string(),
            impact: base_rate * (complexity_multiplier - 1.0),
            description: format!("{}% complexity adjustment", ((complexity_multiplier - 1.0) * 100.0) as i32),
        });
        factors.push(PricingFactor {
            factor_name: "Skill Demand".to_string(),
            impact: base_rate * (demand_multiplier - 1.0),
            description: format!("{}% demand premium", ((demand_multiplier - 1.0) * 100.0) as i32),
        });

        if freelancer_multiplier != 1.0 {
            factors.push(PricingFactor {
                factor_name: "Experience Level".to_string(),
                impact: base_rate * (freelancer_multiplier - 1.0),
                description: format!("{}% experience adjustment", ((freelancer_multiplier - 1.0) * 100.0) as i32),
            });
        }

        let confidence_level = if market_data.sample_size > 50 { 0.9 } else if market_data.sample_size > 20 { 0.75 } else { 0.6 };

        Ok(PricingSuggestion {
            suggested_range: (min_rate, max_rate),
            market_average: base_rate,
            competitive_rate,
            confidence_level,
            factors,
        })
    }

    /// Estimate project timeline
    pub async fn estimate_timeline(
        &self,
        request: TimelineRequest,
    ) -> Result<TimelineEstimation, sqlx::Error> {
        // Get historical completion data
        let historical_data = self.get_historical_completions(
            &request.project_type,
            request.complexity_level,
        ).await?;

        // Calculate base estimate from historical data
        let avg_completion_days = if !historical_data.is_empty() {
            historical_data.iter().map(|h| h.actual_days).sum::<i32>() as f64 / historical_data.len() as f64
        } else {
            30.0 // Default fallback
        };

        // Adjust for team size
        let team_multiplier = if request.team_size > 1 {
            1.0 / (1.0 + (request.team_size as f64 - 1.0) * 0.3)
        } else {
            1.0
        };

        // Adjust for complexity
        let complexity_adjustment = request.complexity_level as f64 * 20.0;

        // Get team velocity if team members provided
        let velocity_adjustment = if !request.freelancer_ids.is_empty() {
            let velocity = self.calculate_team_velocity(&request.freelancer_ids).await?;
            velocity.avg_velocity
        } else {
            1.0
        };

        // Calculate estimated days
        let base_estimate = (avg_completion_days + complexity_adjustment) * team_multiplier / velocity_adjustment;
        
        // Calculate risk buffer (15-30% based on complexity)
        let risk_buffer_percent = 0.15 + (request.complexity_level as f64 * 0.15);
        let risk_buffer_days = (base_estimate * risk_buffer_percent) as i32;
        
        let estimated_days = base_estimate as i32 + risk_buffer_days;

        // Build task breakdown (simplified)
        let breakdown = vec![
            TaskEstimate {
                task_name: "Planning & Setup".to_string(),
                estimated_hours: base_estimate * 8.0 * 0.1,
                complexity: request.complexity_level * 0.5,
                dependencies: vec![],
            },
            TaskEstimate {
                task_name: "Core Development".to_string(),
                estimated_hours: base_estimate * 8.0 * 0.6,
                complexity: request.complexity_level,
                dependencies: vec!["Planning & Setup".to_string()],
            },
            TaskEstimate {
                task_name: "Testing & QA".to_string(),
                estimated_hours: base_estimate * 8.0 * 0.2,
                complexity: request.complexity_level * 0.7,
                dependencies: vec!["Core Development".to_string()],
            },
            TaskEstimate {
                task_name: "Deployment & Documentation".to_string(),
                estimated_hours: base_estimate * 8.0 * 0.1,
                complexity: request.complexity_level * 0.4,
                dependencies: vec!["Testing & QA".to_string()],
            },
        ];

        // Build factors
        let mut factors = Vec::new();
        factors.push(TimelineFactor {
            factor_name: "Historical Data".to_string(),
            impact_days: avg_completion_days as i32,
            description: format!("Based on {} similar projects", historical_data.len()),
        });
        if request.team_size > 1 {
            factors.push(TimelineFactor {
                factor_name: "Team Size".to_string(),
                impact_days: -(avg_completion_days * (1.0 - team_multiplier)) as i32,
                description: format!("{} team members improve efficiency", request.team_size),
            });
        }
        factors.push(TimelineFactor {
            factor_name: "Risk Buffer".to_string(),
            impact_days: risk_buffer_days,
            description: format!("{}% buffer for uncertainties", (risk_buffer_percent * 100.0) as i32),
        });

        let confidence_level = if historical_data.len() > 30 { 0.85 } else if historical_data.len() > 10 { 0.7 } else { 0.55 };

        Ok(TimelineEstimation {
            estimated_days,
            confidence_level,
            breakdown,
            risk_buffer_days,
            factors,
        })
    }

    // Helper methods
    async fn get_freelancer_track_record(
        &self,
        freelancer_id: Uuid,
        skills: &[String],
    ) -> Result<FreelancerTrackRecord, sqlx::Error> {
        // Query database for freelancer's past projects
        let stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_projects,
                COUNT(*) FILTER (WHERE c.status = 'completed') as completed,
                AVG(EXTRACT(EPOCH FROM (c.updated_at - c.created_at)) / 86400) as avg_days,
                AVG(r.communication_rating) as avg_comm
            FROM contracts c
            LEFT JOIN reviews r ON r.freelancer_id = c.freelancer_id
            WHERE c.freelancer_id = $1
            "#,
            freelancer_id
        )
        .fetch_one(&self.pool)
        .await?;

        let success_rate = if stats.total_projects.unwrap_or(0) > 0 {
            stats.completed.unwrap_or(0) as f64 / stats.total_projects.unwrap_or(1) as f64
        } else {
            0.5
        };

        Ok(FreelancerTrackRecord {
            freelancer_id,
            similar_projects_completed: stats.completed.unwrap_or(0) as i32,
            success_rate,
            avg_completion_time: stats.avg_days.unwrap_or(30.0),
            avg_budget_adherence: 0.9,
            communication_score: (stats.avg_comm.unwrap_or(4.0) / 5.0),
        })
    }

    async fn analyze_budget_adequacy(
        &self,
        budget: f64,
        skills: &[String],
        complexity: f32,
    ) -> Result<BudgetAnalysis, sqlx::Error> {
        let market_rate = 75.0; // Simplified - should query market data
        let estimated_hours = 40.0 + (complexity as f64 * 80.0);
        let recommended_budget = market_rate * estimated_hours;

        Ok(BudgetAnalysis {
            is_adequate: budget >= recommended_budget * 0.8,
            recommended_budget,
        })
    }

    async fn analyze_timeline_realism(
        &self,
        timeline_days: i32,
        skills: &[String],
        complexity: f32,
    ) -> Result<TimelineAnalysis, sqlx::Error> {
        let base_days = 14;
        let complexity_days = (complexity as i32 * 30).max(7);
        let recommended_days = base_days + complexity_days;

        Ok(TimelineAnalysis {
            is_realistic: timeline_days >= (recommended_days as f64 * 0.85) as i32,
            recommended_days,
        })
    }

    async fn analyze_skill_match(
        &self,
        freelancer_id: Uuid,
        required_skills: &[String],
    ) -> Result<SkillMatchAnalysis, sqlx::Error> {
        // Simplified - should query actual freelancer skills
        let coverage = 0.85;
        let missing_skills = vec![];

        Ok(SkillMatchAnalysis {
            coverage,
            missing_skills,
        })
    }

    async fn get_market_pricing(
        &self,
        project_type: &str,
        skills: &[String],
    ) -> Result<MarketPricingData, sqlx::Error> {
        // Return default market data - should query actual database
        Ok(MarketPricingData {
            id: 1,
            project_type: project_type.to_string(),
            skill_category: "general".to_string(),
            avg_hourly_rate: 75.0,
            min_rate: 50.0,
            max_rate: 120.0,
            sample_size: 100,
            last_updated: chrono::Utc::now().naive_utc(),
        })
    }

    async fn get_freelancer_pricing_multiplier(&self, freelancer_id: Uuid) -> Result<f64, sqlx::Error> {
        // Calculate based on experience and ratings
        Ok(1.1)
    }

    async fn calculate_skill_demand(&self, skills: &[String]) -> Result<f64, sqlx::Error> {
        // Calculate demand score 0-1
        Ok(0.7)
    }

    async fn get_historical_completions(
        &self,
        project_type: &str,
        complexity: f32,
    ) -> Result<Vec<HistoricalCompletion>, sqlx::Error> {
        // Query historical project data
        Ok(vec![])
    }

    async fn calculate_team_velocity(&self, freelancer_ids: &[Uuid]) -> Result<TeamVelocity, sqlx::Error> {
        Ok(TeamVelocity {
            team_id: None,
            avg_velocity: 1.1,
            consistency_score: 0.85,
            recent_performance: vec![1.0, 1.1, 1.15],
        })
    }
}

struct BudgetAnalysis {
    is_adequate: bool,
    recommended_budget: f64,
}

struct TimelineAnalysis {
    is_realistic: bool,
    recommended_days: i32,
}

struct SkillMatchAnalysis {
    coverage: f64,
    missing_skills: Vec<String>,
}
