
-- KYC Verifications
CREATE TABLE IF NOT EXISTS kyc_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id INTEGER NOT NULL REFERENCES users(id),
    verification_level VARCHAR(20) NOT NULL DEFAULT 'None',
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',
    id_document_url TEXT,
    proof_of_address_url TEXT,
    selfie_url TEXT,
    verification_provider VARCHAR(50),
    provider_verification_id VARCHAR(100),
    verified_at TIMESTAMP,
    expires_at TIMESTAMP,
    rejection_reason TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_kyc_user_id ON kyc_verifications(user_id);
CREATE INDEX idx_kyc_status ON kyc_verifications(status);

-- Fiat Transactions
CREATE TABLE IF NOT EXISTS fiat_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id INTEGER NOT NULL REFERENCES wallets(id),
    transaction_type VARCHAR(20) NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    currency_code VARCHAR(10) NOT NULL,
    payment_method VARCHAR(30) NOT NULL,
    payment_provider VARCHAR(30) NOT NULL,
    provider_transaction_id VARCHAR(100),
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',
    fees DECIMAL(18, 2) DEFAULT 0,
    destination_address TEXT,
    metadata JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP
);

CREATE INDEX idx_fiat_tx_wallet ON fiat_transactions(wallet_id);
CREATE INDEX idx_fiat_tx_status ON fiat_transactions(status);
CREATE INDEX idx_fiat_tx_provider ON fiat_transactions(payment_provider);

-- Currency Swaps
CREATE TABLE IF NOT EXISTS currency_swaps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id INTEGER NOT NULL REFERENCES wallets(id),
    from_currency VARCHAR(10) NOT NULL,
    to_currency VARCHAR(10) NOT NULL,
    from_amount DECIMAL(18, 8) NOT NULL,
    to_amount DECIMAL(18, 8) NOT NULL,
    exchange_rate DECIMAL(18, 8) NOT NULL,
    slippage_tolerance DECIMAL(5, 4) DEFAULT 0.005,
    actual_slippage DECIMAL(5, 4),
    swap_provider VARCHAR(20) NOT NULL,
    tx_hash VARCHAR(66),
    fees DECIMAL(18, 8) DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP
);

CREATE INDEX idx_swaps_wallet ON currency_swaps(wallet_id);
CREATE INDEX idx_swaps_currencies ON currency_swaps(from_currency, to_currency);
CREATE INDEX idx_swaps_created ON currency_swaps(created_at DESC);

-- Auto Conversion Preferences
CREATE TABLE IF NOT EXISTS auto_conversion_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id INTEGER NOT NULL UNIQUE REFERENCES wallets(id),
    enabled BOOLEAN DEFAULT false,
    target_currency VARCHAR(10) NOT NULL,
    minimum_amount DECIMAL(18, 8) DEFAULT 0,
    convert_on_receive BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_auto_conversion_wallet ON auto_conversion_preferences(wallet_id);
