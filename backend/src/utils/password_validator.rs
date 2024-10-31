use once_cell::sync::Lazy;
use regex::Regex;

const PATTERN: &str = r"^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*\p{P})[\p{L}\p{N}\p{P}\p{S}]{12,}$";

static PASSWORD_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(PATTERN).expect("Échec de la compilation de la regex"));

pub fn validate_password(password: &str) -> bool {
  PASSWORD_REGEX.is_match(password)
}
