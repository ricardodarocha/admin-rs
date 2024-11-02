use std::time::Duration;

use crate::infra::result::Result;
use crate::infra::jwt::{create_jwt, jwt_secret};

// Gere um token expirável para um usuário, especifique a duração 
// let token = token("admin", Duration::from_days(15))
pub fn token(user_id: &String, duration: Duration) -> Result<String> {
    let secret = jwt_secret();
    create_jwt(&user_id, secret.as_slice(), duration) 
}
