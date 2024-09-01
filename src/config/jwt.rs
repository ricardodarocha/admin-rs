use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // Token válido por 1 hora

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}

pub fn get_claims(token: &str) -> Option<Claims> {
    match validate_jwt(token) {
        Ok(data) => Some(data.claims),
        Err(_) => None,
    }
}
