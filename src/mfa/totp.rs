use totp_rs::{Secret, TOTP};
use base32::Alphabet;
use rand::Rng;

pub struct MfaService {
    issuer: String,
}

impl MfaService {
    pub fn new(issuer: String) -> Self {
        Self { issuer }
    }

    pub fn generate_secret(&self) -> (String, String) {
        let secret: [u8; 20] = rand::thread_rng().gen();
        let secret_base32 = base32::encode(Alphabet::RFC4648 { padding: true }, &secret);
        
        let totp = TOTP::new(
            totp_rs::Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_vec(),
            Some(self.issuer.clone()),
            "user@example.com".to_string(),
        ).unwrap();

        let qr_code = totp.get_qr_base64()?;
        
        (secret_base32, qr_code)
    }

    pub fn verify_code(&self, secret: &str, code: &str) -> bool {
        let secret = match base32::decode(Alphabet::RFC4648 { padding: true }, secret) {
            Some(s) => s,
            None => return false,
        };

        let totp = TOTP::new(
            totp_rs::Algorithm::SHA1,
            6,
            1,
            30,
            secret,
            Some(self.issuer.clone()),
            "user@example.com".to_string(),
        ).unwrap();

        totp.check_current(code).unwrap_or(false)
    }
}