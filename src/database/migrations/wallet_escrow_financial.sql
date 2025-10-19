
-- Wallet Escrow Tables
CREATE TABLE IF NOT EXISTS wallet_escrows (
    id SERIAL PRIMARY KEY,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id),
    project_id INTEGER NOT NULL,
    contract_id INTEGER NOT NULL,
    amount DECIMAL(18, 8) NOT NULL,
    currency_code VARCHAR(10) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Locked',
    locked_at TIMESTAMP NOT NULL,
    released_at TIMESTAMP,
    refunded_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS escrow_milestones (
    id SERIAL PRIMARY KEY,
    escrow_id INTEGER NOT NULL REFERENCES wallet_escrows(id),
    milestone_id INTEGER NOT NULL,
    amount DECIMAL(18, 8) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',
    released_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS wallet_disputes (
    id SERIAL PRIMARY KEY,
    escrow_id INTEGER NOT NULL REFERENCES wallet_escrows(id),
    reason TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Open',
    resolved_at TIMESTAMP,
    resolution TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Financial Management Tables
CREATE TABLE IF NOT EXISTS income_entries (
    id SERIAL PRIMARY KEY,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id),
    project_id INTEGER,
    client_id INTEGER,
    amount DECIMAL(18, 2) NOT NULL,
    currency_code VARCHAR(10) NOT NULL,
    category VARCHAR(50) NOT NULL,
    description TEXT,
    date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS expense_entries (
    id SERIAL PRIMARY KEY,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id),
    amount DECIMAL(18, 2) NOT NULL,
    currency_code VARCHAR(10) NOT NULL,
    category VARCHAR(50) NOT NULL,
    description TEXT,
    receipt_url TEXT,
    is_business BOOLEAN DEFAULT true,
    date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS budgets (
    id SERIAL PRIMARY KEY,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id),
    name VARCHAR(255) NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    currency_code VARCHAR(10) NOT NULL,
    period VARCHAR(20) NOT NULL,
    alert_threshold DECIMAL(5, 2) DEFAULT 0.80,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS savings_goals (
    id SERIAL PRIMARY KEY,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id),
    name VARCHAR(255) NOT NULL,
    target_amount DECIMAL(18, 2) NOT NULL,
    current_amount DECIMAL(18, 2) DEFAULT 0,
    currency_code VARCHAR(10) NOT NULL,
    deadline DATE,
    auto_contribute BOOLEAN DEFAULT false,
    contribution_amount DECIMAL(18, 2),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS invoices (
    id SERIAL PRIMARY KEY,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id),
    invoice_number VARCHAR(50) UNIQUE NOT NULL,
    client_name VARCHAR(255) NOT NULL,
    client_email VARCHAR(255) NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    currency_code VARCHAR(10) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    due_date DATE NOT NULL,
    issued_date DATE NOT NULL,
    paid_date DATE,
    items JSONB NOT NULL,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_wallet_escrows_wallet ON wallet_escrows(wallet_id);
CREATE INDEX idx_wallet_escrows_project ON wallet_escrows(project_id);
CREATE INDEX idx_escrow_milestones_escrow ON escrow_milestones(escrow_id);
CREATE INDEX idx_income_entries_wallet ON income_entries(wallet_id);
CREATE INDEX idx_income_entries_date ON income_entries(date);
CREATE INDEX idx_expense_entries_wallet ON expense_entries(wallet_id);
CREATE INDEX idx_expense_entries_date ON expense_entries(date);
CREATE INDEX idx_budgets_wallet ON budgets(wallet_id);
CREATE INDEX idx_savings_goals_wallet ON savings_goals(wallet_id);
CREATE INDEX idx_invoices_wallet ON invoices(wallet_id);
