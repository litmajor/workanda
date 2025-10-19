-- Create wallets table
CREATE TABLE IF NOT EXISTS wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    celo_address VARCHAR(42) NOT NULL UNIQUE,
    encrypted_private_key TEXT NOT NULL,
    seed_phrase_hash TEXT NOT NULL,
    is_multi_sig BOOLEAN DEFAULT FALSE,
    multi_sig_threshold INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_wallets_user_id ON wallets(user_id);
CREATE INDEX idx_wallets_celo_address ON wallets(celo_address);

-- Create wallet_signers table for multi-signature support
CREATE TABLE IF NOT EXISTS wallet_signers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id UUID NOT NULL,
    signer_user_id UUID NOT NULL,
    signer_address VARCHAR(42) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_wallet FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE,
    CONSTRAINT fk_signer_user FOREIGN KEY (signer_user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_wallet_signers_wallet_id ON wallet_signers(wallet_id);

-- Create currency_balances table
CREATE TABLE IF NOT EXISTS currency_balances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id UUID NOT NULL,
    currency_code VARCHAR(10) NOT NULL,
    currency_type VARCHAR(20) NOT NULL CHECK (currency_type IN ('fiat', 'stablecoin', 'crypto')),
    balance DECIMAL(36, 18) DEFAULT 0,
    locked_balance DECIMAL(36, 18) DEFAULT 0,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_wallet_balance FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE,
    UNIQUE(wallet_id, currency_code)
);

CREATE INDEX idx_currency_balances_wallet_id ON currency_balances(wallet_id);
CREATE INDEX idx_currency_balances_currency_code ON currency_balances(currency_code);

-- Create transactions table
CREATE TABLE IF NOT EXISTS wallet_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id UUID NOT NULL,
    transaction_hash VARCHAR(66),
    transaction_type VARCHAR(20) NOT NULL CHECK (transaction_type IN ('send', 'receive', 'request', 'internal')),
    from_address VARCHAR(42),
    to_address VARCHAR(42),
    currency_code VARCHAR(10) NOT NULL,
    amount DECIMAL(36, 18) NOT NULL,
    gas_fee DECIMAL(36, 18),
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'confirmed', 'failed', 'cancelled')),
    block_number BIGINT,
    confirmations INTEGER DEFAULT 0,
    metadata JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    confirmed_at TIMESTAMP,
    CONSTRAINT fk_wallet_transaction FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

CREATE INDEX idx_wallet_transactions_wallet_id ON wallet_transactions(wallet_id);
CREATE INDEX idx_wallet_transactions_hash ON wallet_transactions(transaction_hash);
CREATE INDEX idx_wallet_transactions_status ON wallet_transactions(status);
CREATE INDEX idx_wallet_transactions_created_at ON wallet_transactions(created_at DESC);

-- Create payment_requests table
CREATE TABLE IF NOT EXISTS payment_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id UUID NOT NULL,
    requester_user_id UUID NOT NULL,
    payer_user_id UUID,
    currency_code VARCHAR(10) NOT NULL,
    amount DECIMAL(36, 18) NOT NULL,
    description TEXT,
    qr_code_data TEXT,
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'paid', 'cancelled', 'expired')),
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    paid_at TIMESTAMP,
    transaction_id UUID,
    CONSTRAINT fk_wallet_request FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE,
    CONSTRAINT fk_requester_user FOREIGN KEY (requester_user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_payment_transaction FOREIGN KEY (transaction_id) REFERENCES wallet_transactions(id) ON DELETE SET NULL
);

CREATE INDEX idx_payment_requests_wallet_id ON payment_requests(wallet_id);
CREATE INDEX idx_payment_requests_status ON payment_requests(status);

-- Create exchange_rates table for multi-currency support
CREATE TABLE IF NOT EXISTS exchange_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    base_currency VARCHAR(10) NOT NULL,
    quote_currency VARCHAR(10) NOT NULL,
    rate DECIMAL(36, 18) NOT NULL,
    source VARCHAR(50) NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(base_currency, quote_currency, source)
);

CREATE INDEX idx_exchange_rates_base_currency ON exchange_rates(base_currency);
CREATE INDEX idx_exchange_rates_updated_at ON exchange_rates(updated_at DESC);

-- Create wallet_receipts table
CREATE TABLE IF NOT EXISTS wallet_receipts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID NOT NULL,
    receipt_pdf_url TEXT,
    receipt_data JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_receipt_transaction FOREIGN KEY (transaction_id) REFERENCES wallet_transactions(id) ON DELETE CASCADE
);

CREATE INDEX idx_wallet_receipts_transaction_id ON wallet_receipts(transaction_id);
