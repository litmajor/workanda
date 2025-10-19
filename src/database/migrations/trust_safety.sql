
-- Trust Scores Table
CREATE TABLE IF NOT EXISTS user_trust_scores (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    overall_score NUMERIC(5,2) NOT NULL CHECK (overall_score >= 0 AND overall_score <= 100),
    reliability NUMERIC(5,2) NOT NULL CHECK (reliability >= 0 AND reliability <= 100),
    communication NUMERIC(5,2) NOT NULL CHECK (communication >= 0 AND communication <= 100),
    quality NUMERIC(5,2) NOT NULL CHECK (quality >= 0 AND quality <= 100),
    professionalism NUMERIC(5,2) NOT NULL CHECK (professionalism >= 0 AND professionalism <= 100),
    transparency NUMERIC(5,2) NOT NULL CHECK (transparency >= 0 AND transparency <= 100),
    trend VARCHAR(20) NOT NULL CHECK (trend IN ('Improving', 'Stable', 'Declining')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_user_trust_scores_user_id ON user_trust_scores(user_id);
CREATE INDEX idx_user_trust_scores_overall_score ON user_trust_scores(overall_score);
CREATE INDEX idx_user_trust_scores_created_at ON user_trust_scores(created_at);

-- Fraud Alerts Table
CREATE TABLE IF NOT EXISTS fraud_alerts (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    flag_type VARCHAR(50) NOT NULL,
    risk_score NUMERIC(5,2) NOT NULL CHECK (risk_score >= 0 AND risk_score <= 100),
    description TEXT NOT NULL,
    evidence JSONB,
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'reviewing', 'resolved', 'false_positive')),
    reviewed_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    reviewed_at TIMESTAMPTZ
);

CREATE INDEX idx_fraud_alerts_user_id ON fraud_alerts(user_id);
CREATE INDEX idx_fraud_alerts_status ON fraud_alerts(status);
CREATE INDEX idx_fraud_alerts_risk_score ON fraud_alerts(risk_score);
CREATE INDEX idx_fraud_alerts_created_at ON fraud_alerts(created_at);

-- Dispute Risk Assessments Table
CREATE TABLE IF NOT EXISTS dispute_risk_assessments (
    id SERIAL PRIMARY KEY,
    contract_id UUID NOT NULL REFERENCES contracts(id),
    risk_score NUMERIC(5,2) NOT NULL CHECK (risk_score >= 0 AND risk_score <= 100),
    warning_signs JSONB NOT NULL,
    suggested_actions JSONB NOT NULL,
    mediation_recommended BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_dispute_risk_contract_id ON dispute_risk_assessments(contract_id);
CREATE INDEX idx_dispute_risk_score ON dispute_risk_assessments(risk_score);
CREATE INDEX idx_dispute_risk_created_at ON dispute_risk_assessments(created_at);

-- Behavioral Metrics Table (for tracking user behavior over time)
CREATE TABLE IF NOT EXISTS behavioral_metrics (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    metric_type VARCHAR(50) NOT NULL,
    metric_value NUMERIC(10,2) NOT NULL,
    measured_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_behavioral_metrics_user_id ON behavioral_metrics(user_id);
CREATE INDEX idx_behavioral_metrics_type ON behavioral_metrics(metric_type);
CREATE INDEX idx_behavioral_metrics_measured_at ON behavioral_metrics(measured_at);

-- Reports Table (for user-reported issues)
CREATE TABLE IF NOT EXISTS reports (
    id SERIAL PRIMARY KEY,
    reporter_id UUID NOT NULL REFERENCES users(id),
    reported_user_id UUID NOT NULL REFERENCES users(id),
    report_type VARCHAR(50) NOT NULL,
    description TEXT NOT NULL,
    evidence JSONB,
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'investigating', 'resolved', 'dismissed')),
    reviewed_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);

CREATE INDEX idx_reports_reporter_id ON reports(reporter_id);
CREATE INDEX idx_reports_reported_user_id ON reports(reported_user_id);
CREATE INDEX idx_reports_status ON reports(status);
CREATE INDEX idx_reports_created_at ON reports(created_at);
