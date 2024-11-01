use rand::rngs::OsRng;
use rand::seq::SliceRandom;

pub const PASSWORD_LENGTH: usize = 24;
pub const SHARING_SIZE: usize = PASSWORD_LENGTH / 4;
pub const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?";

fn generate_characters(start: char, end: char) -> Vec<u8> {
  (start as u8..=end as u8).collect()
}

pub fn generate_password() -> String {
  let mut rng = OsRng;
  let length = PASSWORD_LENGTH;

  let uppercase = generate_characters('A', 'Z');
  let lowercase = generate_characters('a', 'z');
  let digits = generate_characters('0', '9');

  let mut password = Vec::with_capacity(length);

  for _ in 0..SHARING_SIZE {
    password.push(*uppercase.choose(&mut rng).unwrap());
    password.push(*lowercase.choose(&mut rng).unwrap());
    password.push(*digits.choose(&mut rng).unwrap());
    password.push(*SYMBOLS.choose(&mut rng).unwrap());
  }

  // Fill in the rest of the password in case of non-integer division
  while password.len() < PASSWORD_LENGTH {
    let &category = [
      uppercase.as_slice(),
      lowercase.as_slice(),
      digits.as_slice(),
      SYMBOLS,
    ]
    .choose(&mut rng)
    .unwrap();

    let &next_char = category.choose(&mut rng).unwrap();

    // Non consecutive characters
    if password
      .last()
      .map(|&last| last != next_char)
      .unwrap_or(true)
    {
      password.push(next_char);
    }
  }

  password.shuffle(&mut rng);

  String::from_utf8(password).expect("Error generating password")
}
