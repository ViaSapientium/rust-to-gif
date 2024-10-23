use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
  pub sub: String,
  pub exp: usize,
}

pub fn generate_jwt(email: &str) -> String {
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  let claims = JwtClaims {
    sub: email.to_owned(),
    exp: 10000000000,
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
