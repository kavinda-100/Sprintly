use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{JWT_EXPIRATION_DAYS, JWT_EXPIRATION_DAYS_FALLBACK_IN_MILLISECONDS};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn generate_jwt(user_id: Uuid, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // Calculate the expiration time for the JWT
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(JWT_EXPIRATION_DAYS))
        .map(|t| t.timestamp() as usize)
        .unwrap_or_else(|| {
            eprintln!("Failed to calculate JWT expiration time");
            // Fallback to a default expiration time (e.g., 7 days) if calculation fails
            JWT_EXPIRATION_DAYS_FALLBACK_IN_MILLISECONDS
        });

    // Create the claims for the JWT
    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    // Encode the JWT using the provided secret
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
