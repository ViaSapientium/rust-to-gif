#[cfg(test)]
mod password_validator_tests {
  use super::*;
  use crate::utils::password_generator::{generate_password, PASSWORD_LENGTH, SYMBOLS};

  #[test]
  fn test_password_length() {
    let password = generate_password();
    assert_eq!(
      password.len(),
      PASSWORD_LENGTH,
      "Password length is incorrect"
    );
  }

  #[test]
  fn test_password_contains_all_categories() {
    let password = generate_password();

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| SYMBOLS.contains(&(c as u8)));

    assert!(has_uppercase, "Password lacks uppercase letters");
    assert!(has_lowercase, "Password lacks lowercase letters");
    assert!(has_digit, "Password lacks digits");
    assert!(has_symbol, "Password lacks symbols");
  }

  #[test]
  fn test_password_non_consecutive_characters() {
    let password = generate_password();

    let non_consecutive = password
      .chars()
      .zip(password.chars().skip(1))
      .all(|(a, b)| a != b);

    assert!(non_consecutive, "Password has consecutive characters");
  }
}
