
-- Create teams table
CREATE TABLE IF NOT EXISTS teams (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    leader_id UUID NOT NULL,
    skills TEXT[] NOT NULL DEFAULT '{}',
    available BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create agencies table
CREATE TABLE IF NOT EXISTS agencies (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL,
    team_ids UUID[] NOT NULL DEFAULT '{}',
    verified BOOLEAN NOT NULL DEFAULT false,
    reputation_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    categories TEXT[] NOT NULL DEFAULT '{}',
    projects_completed INTEGER NOT NULL DEFAULT 0,
    avg_delivery_time INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create team_proposals table
CREATE TABLE IF NOT EXISTS team_proposals (
    id UUID PRIMARY KEY,
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id INTEGER NOT NULL,
    bid_amount DOUBLE PRECISION NOT NULL,
    message TEXT NOT NULL,
    proposed_revenue_distribution JSONB NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create revenue_distributions table
CREATE TABLE IF NOT EXISTS revenue_distributions (
    id UUID PRIMARY KEY,
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    contract_id INTEGER NOT NULL,
    total_amount DOUBLE PRECISION NOT NULL,
    distribution_plan JSONB NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create distribution_payments table
CREATE TABLE IF NOT EXISTS distribution_payments (
    id UUID PRIMARY KEY,
    distribution_id UUID NOT NULL REFERENCES revenue_distributions(id) ON DELETE CASCADE,
    freelancer_id UUID NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    transaction_id VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_teams_leader ON teams(leader_id);
CREATE INDEX idx_agencies_owner ON agencies(owner_id);
CREATE INDEX idx_team_proposals_team ON team_proposals(team_id);
CREATE INDEX idx_team_proposals_job ON team_proposals(job_id);
CREATE INDEX idx_revenue_distributions_team ON revenue_distributions(team_id);
CREATE INDEX idx_revenue_distributions_contract ON revenue_distributions(contract_id);
CREATE INDEX idx_distribution_payments_distribution ON distribution_payments(distribution_id);
CREATE INDEX idx_distribution_payments_freelancer ON distribution_payments(freelancer_id);
