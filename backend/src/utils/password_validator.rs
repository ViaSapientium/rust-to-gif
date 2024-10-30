use regex::Regex;

pub fn validate_password(password: &str) -> bool {
  let re = Regex::new(r"^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*\W).{12,}$").unwrap();
  re.is_match(password)
}
