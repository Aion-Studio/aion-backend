use std::env;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use prisma_client_rust::chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatClaims {
    pub combatant_id: String,
    exp: usize, // Expiration time (optional)
}

#[allow(deprecated)]
pub fn create_combat_token(combatant_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(300))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = CombatClaims {
        combatant_id: combatant_id.to_owned(),
        exp: expiration,
    };
    let secret_key = env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
}

pub fn decode_combat_token(token: &str) -> Result<CombatClaims, jsonwebtoken::errors::Error> {
    let secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");
    decode::<CombatClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
