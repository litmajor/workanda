CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    kyc_status BOOLEAN NOT NULL DEFAULT false,
);


CREATE TABLE client_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    company_name VARCHAR(255) NOT NULL,
    tax_id VARCHAR(50),
    kyc_status BOOLEAN NOT NULL DEFAULT false,
    payment_methods JSONB NOT NULL DEFAULT '[]'
);

CREATE TABLE freelancer_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    specialization VARCHAR(255) NOT NULL,
    portfolio_url VARCHAR(255),
    hourly_rate NUMERIC(10,2) NOT NULL,
    kyc_status BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    token VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL
);