use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::env;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use crate::infra::error::Error;
use crate::infra::result::Result;

pub fn jwt_secret() -> Vec<u8> {
    
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "Sl92HaT5XB9NqbGrKxZYtMfBz5AXCSiVKCfaFSVJDE05AqLO8T0lnxUoBgmRlwL".to_string() )
        .into_bytes()
}
   
// Estrutura para as reivindicações (claims) do JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // Subject será o hash do user_id
    exp: usize,  // Data de expiração do token
}

// Função para gerar um hash truncado do user_id
fn hash_user_id(user_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(user_id);
    let result = hasher.finalize();
    hex::encode(result)[2..8].to_string() // Trunca para os primeiros 6 caracteres
}

// Função para criar um novo token JWT com duração customizável
pub fn create_jwt(user_id: &str, secret: &[u8], duration: Duration) -> Result<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Internal Server Error Epoch Failed to get current time")
        + duration;

    let claims = Claims {
        sub: hash_user_id(user_id),
        exp: expiration.as_secs() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
        .map_err(|_| Error::Simple("Erro ao codificar JW  Token".to_owned()))
}

// Função para validar o token JWT
pub fn validate_jwt(token: &str, secret: &[u8]) -> Result<Claims, Error> {
    let validation = Validation::default();
    
    decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
    .map(|data| data.claims).map_err(|err| Error::Simple(err.to_string()))
    
}
