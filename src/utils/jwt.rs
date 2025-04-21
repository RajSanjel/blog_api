use crate::models::auth::TokenClaims;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

pub fn get_rsa_encoding_key() -> Result<EncodingKey, Box<dyn std::error::Error>> {
    let path = std::env::var("PRIVATE_KEY_PATH")?;
    let pem = std::fs::read(path)?;
    Ok(EncodingKey::from_rsa_pem(&pem)?)
}

pub fn jwt_decode(token: String) -> Result<TokenClaims, String> {
    let path = std::env::var("PUBLIC_KEY_PATH").unwrap();
    let pem = std::fs::read(path).unwrap();
    let key = DecodingKey::from_rsa_pem(&pem).unwrap();
    let message = decode::<TokenClaims>(&token, &key, &Validation::new(Algorithm::RS256))
        .map_err(|_| "Invalid or expired token".to_string())?;
    Ok(message.claims)
}
