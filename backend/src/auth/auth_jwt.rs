use crate::auth::auth_dto::JwtClaims;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

pub fn generate_jwt(email: &str) -> String {
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  let expires_in = Duration::days(365);

  let claims = JwtClaims {
    sub: email.to_owned(),
    exp: (Utc::now() + expires_in).timestamp() as usize,
  };
  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .unwrap()
}

pub fn validate_jwt(token: &str) -> bool {
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  let validation = Validation::default();
  decode::<JwtClaims>(
    token,
    &DecodingKey::from_secret(secret.as_ref()),
    &validation,
  )
  .is_ok()
}
