
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::trust_safety::*;
use chrono::Utc;

pub struct TrustSafetyService {
    pool: PgPool,
}

impl TrustSafetyService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Calculate comprehensive trust score for a user
    pub async fn calculate_trust_score(&self, user_id: Uuid) -> Result<TrustScore, sqlx::Error> {
        let behavioral_analysis = self.analyze_user_behavior(user_id).await?;
        
        // Calculate individual components
        let reliability = self.calculate_reliability_score(&behavioral_analysis);
        let communication = self.calculate_communication_score(&behavioral_analysis);
        let quality = behavioral_analysis.quality_consistency;
        let professionalism = self.calculate_professionalism_score(user_id).await?;
        let transparency = self.calculate_transparency_score(&behavioral_analysis);
        
        // Calculate overall score (weighted average)
        let overall_score = (
            reliability * 0.25 +
            communication * 0.20 +
            quality * 0.25 +
            professionalism * 0.15 +
            transparency * 0.15
        );
        
        // Determine trend
        let trend = self.determine_trust_trend(user_id).await?;
        
        let trust_score = TrustScore {
            overall_score,
            components: TrustComponents {
                reliability,
                communication,
                quality,
                professionalism,
                transparency,
            },
            trend,
        };
        
        // Store in database
        self.store_trust_score(user_id, &trust_score).await?;
        
        Ok(trust_score)
    }

    /// Analyze user behavior patterns
    async fn analyze_user_behavior(&self, user_id: Uuid) -> Result<BehavioralAnalysis, sqlx::Error> {
        // Get message response time
        let message_response_time = self.get_avg_message_response_time(user_id).await?;
        
        // Get project completion rate
        let project_completion_rate = self.get_project_completion_rate(user_id).await?;
        
        // Get budget adherence
        let budget_adherence_score = self.get_budget_adherence_score(user_id).await?;
        
        // Get timeline accuracy
        let timeline_accuracy_score = self.get_timeline_accuracy_score(user_id).await?;
        
        // Get client satisfaction
        let client_satisfaction_avg = self.get_client_satisfaction_avg(user_id).await?;
        
        // Get quality consistency
        let quality_consistency = self.get_quality_consistency_score(user_id).await?;
        
        Ok(BehavioralAnalysis {
            user_id,
            message_response_time,
            project_completion_rate,
            budget_adherence_score,
            timeline_accuracy_score,
            client_satisfaction_avg,
            quality_consistency,
        })
    }

    /// Detect fraudulent activity
    pub async fn detect_fraud(&self, user_id: Uuid) -> Result<FraudDetectionResult, sqlx::Error> {
        let mut flags = Vec::new();
        let mut risk_score = 0.0;
        
        // Check for unusual behavior
        if let Some(flag) = self.check_unusual_behavior(user_id).await? {
            risk_score += flag.severity;
            flags.push(flag);
        }
        
        // Check for fake profile indicators
        if let Some(flag) = self.check_fake_profile(user_id).await? {
            risk_score += flag.severity;
            flags.push(flag);
        }
        
        // Check for payment fraud patterns
        if let Some(flag) = self.check_payment_fraud(user_id).await? {
            risk_score += flag.severity;
            flags.push(flag);
        }
        
        // Check for review manipulation
        if let Some(flag) = self.check_review_manipulation(user_id).await? {
            risk_score += flag.severity;
            flags.push(flag);
        }
        
        // Check for bot-like behavior
        if let Some(flag) = self.check_bot_behavior(user_id).await? {
            risk_score += flag.severity;
            flags.push(flag);
        }
        
        // Determine risk level
        let risk_level = match risk_score {
            s if s < 25.0 => FraudRiskLevel::Low,
            s if s < 50.0 => FraudRiskLevel::Medium,
            s if s < 75.0 => FraudRiskLevel::High,
            _ => FraudRiskLevel::Critical,
        };
        
        // Generate recommendations
        let recommended_actions = self.generate_fraud_recommendations(&flags, risk_level);
        let requires_manual_review = risk_score >= 50.0;
        
        // Store fraud alert if risk is medium or higher
        if risk_level != FraudRiskLevel::Low {
            self.store_fraud_alert(user_id, &flags, risk_score).await?;
        }
        
        Ok(FraudDetectionResult {
            user_id,
            risk_level,
            risk_score,
            flags,
            recommended_actions,
            requires_manual_review,
        })
    }

    /// Predict dispute risk for a contract
    pub async fn predict_dispute_risk(&self, contract_id: Uuid) -> Result<DisputeRiskAlert, sqlx::Error> {
        let mut warning_signs = Vec::new();
        let mut risk_score = 0.0;
        
        // Check communication patterns
        if self.has_communication_breakdown(contract_id).await? {
            warning_signs.push("Communication frequency has decreased significantly".to_string());
            risk_score += 20.0;
        }
        
        // Check milestone completion
        if self.has_missed_milestones(contract_id).await? {
            warning_signs.push("Multiple milestones missed or delayed".to_string());
            risk_score += 25.0;
        }
        
        // Check budget issues
        if self.has_budget_disagreements(contract_id).await? {
            warning_signs.push("Budget discussions or disagreements detected".to_string());
            risk_score += 20.0;
        }
        
        // Check for scope creep
        if self.has_scope_creep(contract_id).await? {
            warning_signs.push("Project scope appears to be expanding beyond original agreement".to_string());
            risk_score += 15.0;
        }
        
        // Check quality concerns
        if self.has_quality_concerns(contract_id).await? {
            warning_signs.push("Quality concerns raised in recent communications".to_string());
            risk_score += 20.0;
        }
        
        // Generate suggested actions
        let suggested_actions = self.generate_dispute_prevention_actions(&warning_signs, risk_score);
        let mediation_recommended = risk_score >= 60.0;
        
        let risk_alert = DisputeRiskAlert {
            contract_id,
            risk_level: risk_score,
            warning_signs,
            suggested_actions,
            mediation_recommended,
        };
        
        // Store assessment
        self.store_dispute_risk_assessment(&risk_alert).await?;
        
        Ok(risk_alert)
    }

    // Helper methods for trust score calculation
    fn calculate_reliability_score(&self, analysis: &BehavioralAnalysis) -> f64 {
        (analysis.project_completion_rate * 50.0) + (analysis.timeline_accuracy_score * 0.5)
    }

    fn calculate_communication_score(&self, analysis: &BehavioralAnalysis) -> f64 {
        // Better response time = higher score (inverse relationship)
        let response_score = if analysis.message_response_time < 2.0 {
            100.0
        } else if analysis.message_response_time < 6.0 {
            80.0
        } else if analysis.message_response_time < 24.0 {
            60.0
        } else {
            40.0
        };
        response_score
    }

    async fn calculate_professionalism_score(&self, user_id: Uuid) -> Result<f64, sqlx::Error> {
        // Check for reports, disputes, negative feedback
        let report_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reports WHERE reported_user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);
        
        let base_score = 100.0;
        let penalty = (report_count as f64) * 5.0;
        Ok((base_score - penalty).max(0.0))
    }

    fn calculate_transparency_score(&self, analysis: &BehavioralAnalysis) -> f64 {
        (analysis.budget_adherence_score * 0.6) + (analysis.timeline_accuracy_score * 0.4)
    }

    async fn determine_trust_trend(&self, user_id: Uuid) -> Result<TrustTrend, sqlx::Error> {
        // Compare current score with historical scores
        let recent_scores: Vec<f64> = sqlx::query_scalar(
            "SELECT overall_score FROM user_trust_scores WHERE user_id = $1 ORDER BY created_at DESC LIMIT 3"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();
        
        if recent_scores.len() < 2 {
            return Ok(TrustTrend::Stable);
        }
        
        let diff = recent_scores[0] - recent_scores[recent_scores.len() - 1];
        
        Ok(if diff > 5.0 {
            TrustTrend::Improving
        } else if diff < -5.0 {
            TrustTrend::Declining
        } else {
            TrustTrend::Stable
        })
    }

    // Database helper methods
    async fn get_avg_message_response_time(&self, user_id: Uuid) -> Result<f64, sqlx::Error> {
        sqlx::query_scalar("SELECT COALESCE(AVG(EXTRACT(EPOCH FROM (response_time - sent_at))/3600), 24.0) FROM messages WHERE recipient_id = $1 AND response_time IS NOT NULL")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
    }

    async fn get_project_completion_rate(&self, user_id: Uuid) -> Result<f64, sqlx::Error> {
        sqlx::query_scalar("SELECT COALESCE(COUNT(CASE WHEN status = 'completed' THEN 1 END)::FLOAT / NULLIF(COUNT(*), 0), 0.0) FROM contracts WHERE freelancer_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
    }

    async fn get_budget_adherence_score(&self, user_id: Uuid) -> Result<f64, sqlx::Error> {
        Ok(85.0) // Placeholder - implement actual calculation
    }

    async fn get_timeline_accuracy_score(&self, user_id: Uuid) -> Result<f64, sqlx::Error> {
        Ok(80.0) // Placeholder - implement actual calculation
    }

    async fn get_client_satisfaction_avg(&self, user_id: Uuid) -> Result<f64, sqlx::Error> {
        sqlx::query_scalar("SELECT COALESCE(AVG(rating), 0.0) FROM reviews WHERE freelancer_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
    }

    async fn get_quality_consistency_score(&self, user_id: Uuid) -> Result<f64, sqlx::Error> {
        Ok(85.0) // Placeholder - implement actual calculation
    }

    async fn store_trust_score(&self, user_id: Uuid, score: &TrustScore) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO user_trust_scores (user_id, overall_score, reliability, communication, quality, professionalism, transparency, trend)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(user_id)
        .bind(score.overall_score)
        .bind(score.components.reliability)
        .bind(score.components.communication)
        .bind(score.components.quality)
        .bind(score.components.professionalism)
        .bind(score.components.transparency)
        .bind(format!("{:?}", score.trend))
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // Fraud detection helper methods
    async fn check_unusual_behavior(&self, user_id: Uuid) -> Result<Option<FraudFlag>, sqlx::Error> {
        // Implement unusual behavior detection logic
        Ok(None)
    }

    async fn check_fake_profile(&self, user_id: Uuid) -> Result<Option<FraudFlag>, sqlx::Error> {
        // Check for incomplete profile, stock photos, etc.
        Ok(None)
    }

    async fn check_payment_fraud(&self, user_id: Uuid) -> Result<Option<FraudFlag>, sqlx::Error> {
        // Check for suspicious payment patterns
        Ok(None)
    }

    async fn check_review_manipulation(&self, user_id: Uuid) -> Result<Option<FraudFlag>, sqlx::Error> {
        // Check for suspicious review patterns
        Ok(None)
    }

    async fn check_bot_behavior(&self, user_id: Uuid) -> Result<Option<FraudFlag>, sqlx::Error> {
        // Check for bot-like activity patterns
        Ok(None)
    }

    fn generate_fraud_recommendations(&self, flags: &[FraudFlag], risk_level: FraudRiskLevel) -> Vec<String> {
        let mut actions = Vec::new();
        
        if risk_level == FraudRiskLevel::Critical {
            actions.push("Immediately suspend account pending review".to_string());
        } else if risk_level == FraudRiskLevel::High {
            actions.push("Flag account for manual review".to_string());
            actions.push("Limit withdrawal capabilities".to_string());
        }
        
        for flag in flags {
            match flag.flag_type {
                FraudFlagType::FakeProfile => actions.push("Request identity verification".to_string()),
                FraudFlagType::PaymentFraud => actions.push("Review recent transactions".to_string()),
                FraudFlagType::ReviewManipulation => actions.push("Audit reviews and ratings".to_string()),
                _ => {}
            }
        }
        
        actions
    }

    async fn store_fraud_alert(&self, user_id: Uuid, flags: &[FraudFlag], risk_score: f64) -> Result<(), sqlx::Error> {
        for flag in flags {
            sqlx::query(
                r#"
                INSERT INTO fraud_alerts (user_id, flag_type, risk_score, description, evidence, status)
                VALUES ($1, $2, $3, $4, $5, 'pending')
                "#
            )
            .bind(user_id)
            .bind(format!("{:?}", flag.flag_type))
            .bind(risk_score)
            .bind(&flag.description)
            .bind(serde_json::to_value(&flag.evidence).unwrap())
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    // Dispute prediction helper methods
    async fn has_communication_breakdown(&self, contract_id: Uuid) -> Result<bool, sqlx::Error> {
        // Check message frequency decline
        Ok(false)
    }

    async fn has_missed_milestones(&self, contract_id: Uuid) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM milestones WHERE project_id = (SELECT project_id FROM contracts WHERE id = $1) AND due_date < NOW() AND completion_status = false"
        )
        .bind(contract_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(count > 0)
    }

    async fn has_budget_disagreements(&self, contract_id: Uuid) -> Result<bool, sqlx::Error> {
        // Analyze message content for budget-related keywords
        Ok(false)
    }

    async fn has_scope_creep(&self, contract_id: Uuid) -> Result<bool, sqlx::Error> {
        // Compare current scope with original
        Ok(false)
    }

    async fn has_quality_concerns(&self, contract_id: Uuid) -> Result<bool, sqlx::Error> {
        // Analyze feedback and communications
        Ok(false)
    }

    fn generate_dispute_prevention_actions(&self, warning_signs: &[String], risk_score: f64) -> Vec<String> {
        let mut actions = Vec::new();
        
        if risk_score >= 60.0 {
            actions.push("Contact both parties to schedule mediation call".to_string());
        }
        
        if warning_signs.iter().any(|s| s.contains("Communication")) {
            actions.push("Send communication reminder to both parties".to_string());
        }
        
        if warning_signs.iter().any(|s| s.contains("milestone")) {
            actions.push("Review and adjust milestone timeline".to_string());
        }
        
        if warning_signs.iter().any(|s| s.contains("Budget")) {
            actions.push("Clarify budget and payment terms".to_string());
        }
        
        actions.push("Document all agreements in writing".to_string());
        
        actions
    }

    async fn store_dispute_risk_assessment(&self, alert: &DisputeRiskAlert) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO dispute_risk_assessments (contract_id, risk_score, warning_signs, suggested_actions, mediation_recommended)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(alert.contract_id)
        .bind(alert.risk_level)
        .bind(serde_json::to_value(&alert.warning_signs).unwrap())
        .bind(serde_json::to_value(&alert.suggested_actions).unwrap())
        .bind(alert.mediation_recommended)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
