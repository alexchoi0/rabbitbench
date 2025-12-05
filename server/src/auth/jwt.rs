use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Missing authorization")]
    MissingAuth,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub exp: usize,
    pub iat: usize,
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, AuthError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|e| {
            tracing::debug!("JWT validation error: {:?}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::InvalidToken,
            }
        })
}

pub fn hash_api_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, result)
}

pub fn generate_api_token() -> String {
    use uuid::Uuid;
    format!("rb_{}", Uuid::new_v4().simple())
}

pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
    auth_header
        .strip_prefix("Bearer ")
        .or_else(|| auth_header.strip_prefix("bearer "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_api_token() {
        let token = "rb_test123";
        let hash = hash_api_token(token);
        assert!(!hash.is_empty());

        let hash2 = hash_api_token(token);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_extract_bearer_token() {
        assert_eq!(extract_bearer_token("Bearer abc123"), Some("abc123"));
        assert_eq!(extract_bearer_token("bearer abc123"), Some("abc123"));
        assert_eq!(extract_bearer_token("Basic abc123"), None);
    }
}
