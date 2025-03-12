use jsonwebtoken::{Header, encode, EncodingKey};
use chrono::Duration;

pub fn generate_token(user: &User) -> String {
    let now = Utc::now();
    let claims = json!({
        "sub": user.id,
        "email": user.email,
        "role": user.role,
        "exp": (now + Duration::days(7)).timestamp()
    });

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref())
    ).unwrap()
}