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

/// This function make the division by 256 of a big number stored in a Vec
/// and returns the modulo of the division.
/// The given Vec is modified and becomes the result of the division.
fn divide_by_58(input: &mut Vec<u8>, start: uint) -> u16 {
  let mut remainder = 0;

  for x in input.mut_iter().skip(start) {
    let num_base256: u16 = (*x as u16) & 0xFF;
    let temp = remainder * BASE_256 + num_base256;
    *x = (temp / BASE_58) as u8;
    remainder = temp % BASE_58;
  }

  remainder
}

/// This function make the division by 58 of a big number stored in a Vec
/// and returns the modulo of the division.
/// The given Vec is modified and becomes the result of the division.
fn divide_by_256(input: &mut Vec<u8>, start: uint) -> u16 {
  let mut remainder = 0;

  for x in input.mut_iter().skip(start) {
    let num_base58: u16 = (*x as u16) & 0xFF;
    let temp = remainder * BASE_58 + num_base58;
    *x = (temp / BASE_256) as u8;
    remainder = temp % BASE_256;
  }

  remainder
}

#[cfg(test)]
mod tests {

  use super::divide_by_256;
  use super::divide_by_58;

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

}
