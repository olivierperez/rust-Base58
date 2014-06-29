#![crate_id = "base58"]
#![crate_type = "rlib"]
#![license = "Apache License, Version 2.0"]
#![deny(missing_doc)]

//! Rust-Base58 provides functions to encode and decode base58 values.

static BASE_256:u16 = 256;
static BASE_58:u16 = 58;

/// Encode a String into base58 String.
pub fn encode_base58 (input: String) -> String {
  let mut copied = input.into_bytes();
  let mut output:Vec<u8> = Vec::with_capacity(copied.len() * 138 / 100 + 1);
  let len = copied.len();

  if len == 0 {
    return String::new();
  }

  let mut count = 0;
  while count < len {
    let modulo = divide_mode58(&mut copied, count);
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

/// This function make the division of a big number stored in a Vec
/// and returns the modulo of the division.
/// The given Vec is modified and becomes the result of the division.
fn divide_mode58(input: &mut Vec<u8>, start: uint) -> u16 {
  let mut remainder = 0;

  for x in input.mut_iter().skip(start) {
    let num_base256: u16 = (*x as u16) & 0xFF;
    let temp = remainder * BASE_256 + num_base256;
    *x = (temp / BASE_58) as u8;
    remainder = temp % BASE_58;
  }

  remainder
}

#[cfg(test)] // TODO Put this test to test.rs file
mod bench {
  extern crate test;
  use self::test::Bencher;
  use super::encode_base58;

  #[bench]
  fn encode_lorem_ispum(b: &mut Bencher) {
    b.iter(|| encode_base58(String::from_str("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")));
  }
}

#[cfg(test)]
mod tests {
  mod tests_encode_base58 {

    use super::super::encode_base58;

    #[test] // TODO Put this test to test.rs file
    fn should_encode_from_string_to_base58_string() {
      // Given
      let empty = String::from_str("");
      let azerty = String::from_str("azerty");
      let lorem = String::from_str("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");

      // When
      let empty_encoded = encode_base58(empty);
      let azerty_encoded = encode_base58(azerty);
      let lorem_encoded = encode_base58(lorem);

      // Then
      assert_eq!(empty_encoded, String::from_str(""));
      assert_eq!(azerty_encoded, String::from_str("qYPmmAqv"));
      assert_eq!(lorem_encoded, String::from_str("RKDitRLwUhnQCAmM9hhenQXRAiKrL3ByvJ4CYouvhmw6yV8fapqyM95bN6KQVJbKvEN3uHeMdP9G7b5HNxmwRGqGibzcjrKaRLL2Nn1GFZRRv2Q8AvQU7MtRagLLzcZcwBDzkMqEpbRy9CvGr3BE7t2rH4wXmdExSGNKqV5XxAG7QT2S4ZQ4nzom8QFmoUgkfj8ykPye3z8FU9gJzSf2cDRA1ecYwFJtNdycgKTYCQ92pcfq95DoWfTBhnJfkKZXe1Dv3guVKk5otHz4XtWFJjcSC2GLrkRhTAEtgkySmaePs5YyoQHKb8PU5paS3dxAWc7sV5rqwrJbSbj7ybK6FQDN2mgbBxy7Wc7CnX6Q3Jm4B8cmo4oGixuHKdufABEs4FM3PAhF5CTk9xFsyw3CVJGxE2sMyC5EKnAv5Zc4kGfzzTxQe7VZrm188XqGZKZVRhjSW9CeceAf9bvqefLahgGiKps13595WSErpo1HpHb6Qt22QLSMJFGB2Aq7asXXUj55wPxCfVCz8NiqewcNpxF9bvjW9xsaMGqgD4dpgrBFzUw6irds1X481DHnRDi98yAhQJVUoxP8WSm3qJ93qhq8Hx7Eg4aiy"));
    }
  }

  mod tests_divide_mode58 {
    use super::super::divide_mode58;

    #[test]
    fn should_divide_zero_without_modulo() {
      // Given
      let mut input = vec!(0u8, 0, 0, 0);

      // When
      let modulo = divide_mode58(&mut input, 0);

      // Then
      assert_eq!(*input.get(0), 0);
      assert_eq!(*input.get(1), 0);
      assert_eq!(*input.get(2), 0);
      assert_eq!(*input.get(3), 0);
      assert_eq!(modulo, 0);
    }

    #[test]
    fn should_divide_100_43_without_modulo() {
      // Given
      let mut input = vec!(100u8, 43);

      // When
      let modulo = divide_mode58(&mut input, 0);

      // Then
      assert_eq!(*input.get(0), 1);
      assert_eq!(*input.get(1), 186);
      assert_eq!(modulo, 7);
    }
  }

}
