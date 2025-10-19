
-- Market Pricing Data
CREATE TABLE IF NOT EXISTS market_pricing_data (
    id SERIAL PRIMARY KEY,
    project_type VARCHAR(100) NOT NULL,
    skill_category VARCHAR(100) NOT NULL,
    avg_hourly_rate DECIMAL(10,2) NOT NULL,
    min_rate DECIMAL(10,2) NOT NULL,
    max_rate DECIMAL(10,2) NOT NULL,
    sample_size INTEGER NOT NULL DEFAULT 0,
    last_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_market_pricing_type ON market_pricing_data(project_type);
CREATE INDEX idx_market_pricing_skill ON market_pricing_data(skill_category);

-- Historical Project Completions
CREATE TABLE IF NOT EXISTS historical_completions (
    id SERIAL PRIMARY KEY,
    project_type VARCHAR(100) NOT NULL,
    complexity_level REAL NOT NULL,
    team_size INTEGER NOT NULL DEFAULT 1,
    actual_days INTEGER NOT NULL,
    estimated_days INTEGER NOT NULL,
    success BOOLEAN NOT NULL DEFAULT true,
    budget DECIMAL(15,2),
    final_cost DECIMAL(15,2),
    completed_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_historical_type ON historical_completions(project_type);
CREATE INDEX idx_historical_complexity ON historical_completions(complexity_level);
CREATE INDEX idx_historical_success ON historical_completions(success);

-- Risk Assessments (store predictions)
CREATE TABLE IF NOT EXISTS risk_assessments (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES jobs(id),
    freelancer_id UUID REFERENCES users(id),
    risk_level VARCHAR(20) NOT NULL,
    success_probability DECIMAL(5,4) NOT NULL,
    risk_factors JSONB NOT NULL,
    recommendations JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_risk_project ON risk_assessments(project_id);
CREATE INDEX idx_risk_freelancer ON risk_assessments(freelancer_id);
CREATE INDEX idx_risk_level ON risk_assessments(risk_level);

-- Pricing Suggestions (store for analysis)
CREATE TABLE IF NOT EXISTS pricing_suggestions (
    id SERIAL PRIMARY KEY,
    project_type VARCHAR(100) NOT NULL,
    freelancer_id UUID REFERENCES users(id),
    suggested_min DECIMAL(10,2) NOT NULL,
    suggested_max DECIMAL(10,2) NOT NULL,
    market_average DECIMAL(10,2) NOT NULL,
    competitive_rate DECIMAL(10,2) NOT NULL,
    confidence_level DECIMAL(5,4) NOT NULL,
    factors JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_pricing_type ON pricing_suggestions(project_type);
CREATE INDEX idx_pricing_freelancer ON pricing_suggestions(freelancer_id);

-- Timeline Estimations (store for learning)
CREATE TABLE IF NOT EXISTS timeline_estimations (
    id SERIAL PRIMARY KEY,
    project_type VARCHAR(100) NOT NULL,
    complexity_level REAL NOT NULL,
    team_size INTEGER NOT NULL DEFAULT 1,
    estimated_days INTEGER NOT NULL,
    risk_buffer_days INTEGER NOT NULL,
    confidence_level DECIMAL(5,4) NOT NULL,
    breakdown JSONB NOT NULL,
    factors JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_timeline_type ON timeline_estimations(project_type);
CREATE INDEX idx_timeline_complexity ON timeline_estimations(complexity_level);

-- Seed some sample market pricing data
INSERT INTO market_pricing_data (project_type, skill_category, avg_hourly_rate, min_rate, max_rate, sample_size)
VALUES 
    ('web_development', 'frontend', 75.00, 50.00, 120.00, 150),
    ('web_development', 'backend', 85.00, 60.00, 140.00, 120),
    ('mobile_development', 'ios', 95.00, 70.00, 160.00, 80),
    ('mobile_development', 'android', 90.00, 65.00, 150.00, 85),
    ('data_science', 'machine_learning', 110.00, 80.00, 180.00, 60),
    ('design', 'ui_ux', 70.00, 45.00, 110.00, 100),
    ('devops', 'cloud', 100.00, 75.00, 170.00, 70);
