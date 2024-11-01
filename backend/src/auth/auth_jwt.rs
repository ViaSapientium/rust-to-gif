use crate::auth::dto::JwtClaims;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

pub fn generate_jwt(email: &str) -> String {
  // Load JWT_SECRET from environment variables
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

  // Calculate expiration date (in 24 hours)
  let expiration = Utc::now()
    .checked_add_signed(Duration::hours(24))
    .expect("valid timestamp")
    .timestamp() as usize;

  // Create JWT claims
  let claims = JwtClaims {
    sub: email.to_owned(),
    exp: expiration,
  };

  // Generate the JWT token
  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .expect("Token encoding failed")
}

pub fn validate_jwt(token: &str) -> bool {
  // Load JWT_SECRET from environment variables
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

  // Validate the JWT token
  let token_data = decode::<JwtClaims>(
    token,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  );
  token_data.is_ok()
}
