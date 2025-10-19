
-- Create wallet status enum
CREATE TYPE wallet_status AS ENUM ('Active', 'Frozen', 'Closed');

-- Create transaction type enum
CREATE TYPE transaction_type AS ENUM (
    'Deposit', 
    'Withdrawal', 
    'EscrowLock', 
    'EscrowRelease', 
    'EscrowRefund', 
    'Transfer', 
    'Fee', 
    'Refund', 
    'Reward'
);

-- Create transaction status enum
CREATE TYPE transaction_status AS ENUM ('Pending', 'Completed', 'Failed', 'Cancelled');

-- Create wallets table
CREATE TABLE IF NOT EXISTS wallets (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    currency VARCHAR(10) NOT NULL DEFAULT 'USD',
    balance DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    available_balance DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    locked_balance DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    wallet_address VARCHAR(255),
    is_primary BOOLEAN NOT NULL DEFAULT false,
    status wallet_status NOT NULL DEFAULT 'Active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, currency),
    CHECK (balance >= 0),
    CHECK (available_balance >= 0),
    CHECK (locked_balance >= 0),
    CHECK (balance = available_balance + locked_balance)
);

-- Create wallet transactions table
CREATE TABLE IF NOT EXISTS wallet_transactions (
    id SERIAL PRIMARY KEY,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id) ON DELETE CASCADE,
    transaction_type transaction_type NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    description TEXT NOT NULL,
    reference_id VARCHAR(255),
    reference_type VARCHAR(50),
    balance_before DECIMAL(15, 2) NOT NULL,
    balance_after DECIMAL(15, 2) NOT NULL,
    status transaction_status NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_wallets_user_id ON wallets(user_id);
CREATE INDEX idx_wallets_currency ON wallets(currency);
CREATE INDEX idx_wallets_status ON wallets(status);
CREATE INDEX idx_wallet_transactions_wallet_id ON wallet_transactions(wallet_id);
CREATE INDEX idx_wallet_transactions_type ON wallet_transactions(transaction_type);
CREATE INDEX idx_wallet_transactions_status ON wallet_transactions(status);
CREATE INDEX idx_wallet_transactions_created_at ON wallet_transactions(created_at DESC);
CREATE INDEX idx_wallet_transactions_reference ON wallet_transactions(reference_id, reference_type);

-- Create trigger to update wallet updated_at
CREATE OR REPLACE FUNCTION update_wallet_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER wallet_updated_at_trigger
    BEFORE UPDATE ON wallets
    FOR EACH ROW
    EXECUTE FUNCTION update_wallet_timestamp();

-- Insert default USD wallets for existing users
INSERT INTO wallets (user_id, currency, is_primary, status)
SELECT id, 'USD', true, 'Active'
FROM users
WHERE NOT EXISTS (
    SELECT 1 FROM wallets WHERE wallets.user_id = users.id
);
