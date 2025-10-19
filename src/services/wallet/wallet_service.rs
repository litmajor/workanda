use actix_web::{web, HttpResponse, Result};
use alloy::primitives::{Address, U256};
use bip39::{Mnemonic, Language};
use bip32::{XPrv, Prefix};
use rand_core::OsRng;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, password_hash::SaltString};
use sha2::{Sha256, Digest};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum WalletError {
    DatabaseError(String),
    CryptoError(String),
    ValidationError(String),
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            WalletError::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
            WalletError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for WalletError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub celo_address: String,
    pub is_multi_sig: bool,
    pub multi_sig_threshold: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreationResponse {
    pub wallet: Wallet,
    pub seed_phrase: Vec<String>,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: Uuid,
    pub is_multi_sig: Option<bool>,
    pub multi_sig_threshold: Option<i32>,
    pub encryption_password: String,
}

#[derive(Debug, Deserialize)]
pub struct RecoverWalletRequest {
    pub user_id: Uuid,
    pub seed_phrase: Vec<String>,
    pub encryption_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportWalletRequest {
    pub user_id: Uuid,
    pub private_key: String,
    pub encryption_password: String,
}

pub struct WalletService {
    pool: PgPool,
}

impl WalletService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_wallet(
        &self,
        request: CreateWalletRequest,
    ) -> Result<WalletCreationResponse, WalletError> {
        let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
        
        let seed = mnemonic.to_seed("");
        
        let root_xprv = XPrv::new(&seed)
            .map_err(|e| WalletError::CryptoError(format!("Failed to generate root key: {}", e)))?;
        
        let derivation_path = "m/44'/52752'/0'/0/0";
        let child_xprv = XPrv::derive_from_path(&seed, &derivation_path.parse()
            .map_err(|e| WalletError::CryptoError(format!("Invalid derivation path: {}", e)))?)
            .map_err(|e| WalletError::CryptoError(format!("Key derivation failed: {}", e)))?;
        
        let private_key_bytes = child_xprv.private_key().to_bytes();
        let private_key_hex = hex::encode(&private_key_bytes);
        
        let public_key = child_xprv.public_key();
        let public_key_bytes = public_key.to_bytes();
        
        let mut hasher = Sha256::new();
        hasher.update(&public_key_bytes);
        let hash = hasher.finalize();
        let address_bytes = &hash[12..];
        let celo_address = format!("0x{}", hex::encode(address_bytes));
        
        let encrypted_key = self.encrypt_private_key(&private_key_hex, &request.encryption_password)?;
        
        let seed_phrase_hash = self.hash_seed_phrase(&mnemonic.to_string())?;
        
        let wallet_id = Uuid::new_v4();
        
        sqlx::query!(
            r#"
            INSERT INTO wallets (id, user_id, celo_address, encrypted_private_key, seed_phrase_hash, is_multi_sig, multi_sig_threshold)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            wallet_id,
            request.user_id,
            celo_address,
            encrypted_key,
            seed_phrase_hash,
            request.is_multi_sig.unwrap_or(false),
            request.multi_sig_threshold
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to save wallet: {}", e)))?;
        
        self.initialize_currency_balances(wallet_id).await?;
        
        let wallet = Wallet {
            id: wallet_id,
            user_id: request.user_id,
            celo_address: celo_address.clone(),
            is_multi_sig: request.is_multi_sig.unwrap_or(false),
            multi_sig_threshold: request.multi_sig_threshold,
        };
        
        Ok(WalletCreationResponse {
            wallet,
            seed_phrase: mnemonic.word_iter().map(|s| s.to_string()).collect(),
            address: celo_address,
        })
    }

    pub async fn recover_wallet(
        &self,
        request: RecoverWalletRequest,
    ) -> Result<Wallet, WalletError> {
        let mnemonic_str = request.seed_phrase.join(" ");
        let mnemonic = Mnemonic::parse_in(Language::English, &mnemonic_str)
            .map_err(|e| WalletError::ValidationError(format!("Invalid seed phrase: {}", e)))?;
        
        let seed = mnemonic.to_seed("");
        
        let derivation_path = "m/44'/52752'/0'/0/0";
        let child_xprv = XPrv::derive_from_path(&seed, &derivation_path.parse()
            .map_err(|e| WalletError::CryptoError(format!("Invalid derivation path: {}", e)))?)
            .map_err(|e| WalletError::CryptoError(format!("Key derivation failed: {}", e)))?;
        
        let private_key_bytes = child_xprv.private_key().to_bytes();
        let private_key_hex = hex::encode(&private_key_bytes);
        
        let public_key = child_xprv.public_key();
        let public_key_bytes = public_key.to_bytes();
        
        let mut hasher = Sha256::new();
        hasher.update(&public_key_bytes);
        let hash = hasher.finalize();
        let address_bytes = &hash[12..];
        let celo_address = format!("0x{}", hex::encode(address_bytes));
        
        let encrypted_key = self.encrypt_private_key(&private_key_hex, &request.encryption_password)?;
        let seed_phrase_hash = self.hash_seed_phrase(&mnemonic.to_string())?;
        
        let wallet_id = Uuid::new_v4();
        
        sqlx::query!(
            r#"
            INSERT INTO wallets (id, user_id, celo_address, encrypted_private_key, seed_phrase_hash, is_multi_sig, multi_sig_threshold)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            wallet_id,
            request.user_id,
            celo_address,
            encrypted_key,
            seed_phrase_hash,
            false,
            None::<i32>
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to save recovered wallet: {}", e)))?;
        
        self.initialize_currency_balances(wallet_id).await?;
        
        Ok(Wallet {
            id: wallet_id,
            user_id: request.user_id,
            celo_address,
            is_multi_sig: false,
            multi_sig_threshold: None,
        })
    }

    pub async fn import_wallet(
        &self,
        request: ImportWalletRequest,
    ) -> Result<Wallet, WalletError> {
        let private_key_bytes = hex::decode(&request.private_key.trim_start_matches("0x"))
            .map_err(|e| WalletError::ValidationError(format!("Invalid private key format: {}", e)))?;
        
        if private_key_bytes.len() != 32 {
            return Err(WalletError::ValidationError("Private key must be 32 bytes".to_string()));
        }
        
        let encrypted_key = self.encrypt_private_key(&request.private_key, &request.encryption_password)?;
        
        let wallet_id = Uuid::new_v4();
        let placeholder_address = format!("0x{}", hex::encode(&private_key_bytes[..20]));
        let placeholder_hash = self.hash_seed_phrase("imported_wallet")?;
        
        sqlx::query!(
            r#"
            INSERT INTO wallets (id, user_id, celo_address, encrypted_private_key, seed_phrase_hash, is_multi_sig, multi_sig_threshold)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            wallet_id,
            request.user_id,
            placeholder_address,
            encrypted_key,
            placeholder_hash,
            false,
            None::<i32>
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to import wallet: {}", e)))?;
        
        self.initialize_currency_balances(wallet_id).await?;
        
        Ok(Wallet {
            id: wallet_id,
            user_id: request.user_id,
            celo_address: placeholder_address,
            is_multi_sig: false,
            multi_sig_threshold: None,
        })
    }

    fn encrypt_private_key(&self, private_key: &str, password: &str) -> Result<String, WalletError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let mut key_bytes = [0u8; 32];
        argon2.hash_password_into(password.as_bytes(), salt.as_bytes(), &mut key_bytes)
            .map_err(|e| WalletError::CryptoError(format!("Password hashing failed: {}", e)))?;
        
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|e| WalletError::CryptoError(format!("Cipher initialization failed: {}", e)))?;
        
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher.encrypt(nonce, private_key.as_bytes())
            .map_err(|e| WalletError::CryptoError(format!("Encryption failed: {}", e)))?;
        
        let mut result = Vec::new();
        result.extend_from_slice(salt.as_bytes());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(base64::encode(&result))
    }

    fn hash_seed_phrase(&self, seed_phrase: &str) -> Result<String, WalletError> {
        let mut hasher = Sha256::new();
        hasher.update(seed_phrase.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    async fn initialize_currency_balances(&self, wallet_id: Uuid) -> Result<(), WalletError> {
        let currencies = vec![
            ("USD", "fiat"),
            ("EUR", "fiat"),
            ("GBP", "fiat"),
            ("NGN", "fiat"),
            ("KES", "fiat"),
            ("ZAR", "fiat"),
            ("GHS", "fiat"),
            ("UGX", "fiat"),
            ("cUSD", "stablecoin"),
            ("cEUR", "stablecoin"),
            ("cREAL", "stablecoin"),
            ("CELO", "crypto"),
            ("BTC", "crypto"),
            ("ETH", "crypto"),
            ("USDT", "stablecoin"),
            ("USDC", "stablecoin"),
        ];
        
        for (code, currency_type) in currencies {
            sqlx::query!(
                r#"
                INSERT INTO currency_balances (wallet_id, currency_code, currency_type, balance, locked_balance)
                VALUES ($1, $2, $3, 0, 0)
                "#,
                wallet_id,
                code,
                currency_type
            )
            .execute(&self.pool)
            .await
            .map_err(|e| WalletError::DatabaseError(format!("Failed to initialize currency: {}", e)))?;
        }
        
        Ok(())
    }

    pub async fn get_wallet_by_user(&self, user_id: Uuid) -> Result<Option<Wallet>, WalletError> {
        let wallet = sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, celo_address, is_multi_sig, multi_sig_threshold
            FROM wallets
            WHERE user_id = $1
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch wallet: {}", e)))?;
        
        Ok(wallet)
    }

    pub async fn get_wallet_by_address(&self, address: &str) -> Result<Option<Wallet>, WalletError> {
        let wallet = sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, celo_address, is_multi_sig, multi_sig_threshold
            FROM wallets
            WHERE celo_address = $1
            LIMIT 1
            "#,
            address
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch wallet: {}", e)))?;
        
        Ok(wallet)
    }
}
