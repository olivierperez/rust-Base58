#![crate_id = "base58"]
#![crate_type = "rlib"]
#![license = "Apache License, Version 2.0"]
#![deny(missing_doc)]

//! Rust-Base58 provides functions to encode and decode base58 values.

static BASE_256:u16 = 256;
static BASE_58:u16 = 58;

/// Encode a String into base58 String.
/// 
/// ```rust{.example}
/// extern crate base58;
/// use base58::encode_base58;
/// 
/// fn main() {
///   let encoded = encode_base58(String::from_str("azerty"));
///   println!("encoded: {}",encoded); // will print "encoded: qYPmmAqv"
/// }
/// ```
pub fn encode_base58 (input: String) -> String {
  let mut copied = input.into_bytes();
  let mut output:Vec<u8> = Vec::with_capacity(copied.len() * 138 / 100 + 1);
  let len = copied.len();

  if len == 0 {
    return String::new();
  }

  let mut count = 0;
  while count < len {
    let modulo = divide_by_58(&mut copied, count);
    let letter = from_alphabet(modulo);
    output.push(letter);

    if *copied.get(count) == 0 {
      count += 1;
    }
  }

  output.reverse();

  match String::from_utf8(output) {
    Ok(result) => result,
    Err(e) => fail!(e)
  }
}

/// Give the letter from the base58 alphabet
fn from_alphabet(position: u16) -> u8 {
  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".char_at(position as uint) as u8
}

/// Give the index of a letter in the alphabet
fn index_of_letter(letter: char) -> Option<uint> {
  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".find(letter)
}

/// This function make the division by 256 of a big number stored in a Vec
/// and returns the modulo of the division.
/// The given Vec is modified and becomes the result of the division.
fn divide_by_58(input: &mut Vec<u8>, start: uint) -> u16 {
  division(input, start, BASE_58, BASE_256)
}

/// This function make the division by 58 of a big number stored in a Vec
/// and returns the modulo of the division.
/// The given Vec is modified and becomes the result of the division.
fn divide_by_256(input: &mut Vec<u8>, start: uint) -> u16 {
  division(input, start, BASE_256, BASE_58)
}

/// This function make the division by [divider] of a big number [base] based stored in a Vec
/// and returns the modulo of the division.
/// The given Vec is modified and becomes the result of the division.
fn division(input: &mut Vec<u8>, start: uint, divider: u16, base: u16) -> u16 {
  let mut remainder = 0;

  for x in input.mut_iter().skip(start) {
    let num_base58: u16 = (*x as u16) & 0xFF;
    let temp = remainder * base + num_base58;
    *x = (temp / divider) as u8;
    remainder = temp % divider;
  }

  remainder
}

#[cfg(test)]
mod tests {

  use super::divide_by_256;
  use super::divide_by_58;
  use super::from_alphabet;
  use super::index_of_letter;

  /* Divisions by 256 */

  #[test]
  fn should_divide_zero_by_58_without_modulo() {
    // Given
    let mut input = vec!(0u8, 0, 0, 0);

    // When
    let modulo = divide_by_58(&mut input, 0);

    // Then
    assert_eq!(*input.get(0), 0);
    assert_eq!(*input.get(1), 0);
    assert_eq!(*input.get(2), 0);
    assert_eq!(*input.get(3), 0);
    assert_eq!(modulo, 0);
  }

  #[test]
  fn should_divide_100_43_by_58_with_modulo_7() {
    // Given
    let mut input = vec!(100u8, 43);

    // When
    let modulo = divide_by_58(&mut input, 0);

    // Then
    assert_eq!(*input.get(0), 1);
    assert_eq!(*input.get(1), 186);
    assert_eq!(modulo, 7);
  }

  /* Divisions by 58 */

  #[test]
  fn should_divide_zero_by_256_without_modulo() {
    // Given
    let mut input = vec!(0u8, 0, 0, 0);

    // When
    let modulo = divide_by_256(&mut input, 0);

    // Then
    assert_eq!(*input.get(0), 0);
    assert_eq!(*input.get(1), 0);
    assert_eq!(*input.get(2), 0);
    assert_eq!(*input.get(3), 0);
    assert_eq!(modulo, 0);
  }

  #[test]
  fn should_divide_100_43_by_256_with_modulo_228() {
    // Given
    let mut input = vec!(13u8, 12, 88);

    // When
    let modulo = divide_by_256(&mut input, 0);

    // Then
    assert_eq!(*input.get(0), 0);
    assert_eq!(*input.get(1), 2);
    assert_eq!(*input.get(2), 57);
    assert_eq!(modulo, 228);
  }

  #[test]
  fn should_find_right_letters_from_alphabet() {
    // Given

    // When
    let letter_1 = from_alphabet(0);
    let letter_A = from_alphabet(9);

    // Then
    assert_eq!(letter_1, 49u8);
    assert_eq!(letter_A, 65u8);
  }

  #[test]
  fn should_find_right_index_into_alphabet() {
    // Given

    // When
    let index_1 = index_of_letter('1').unwrap_or(255);
    let index_A = index_of_letter('A').unwrap_or(255);

    // Then
    assert_eq!(index_1, 0);
    assert_eq!(index_A, 9);
  }

}
