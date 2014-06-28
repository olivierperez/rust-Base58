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
  let mut output:Vec<u8> = vec!();
  let len = copied.len();

  if len > 0 {
	  while *copied.get(len - 1) != 0 {
	    let modulo = divide_mode58(&mut copied);
			let letter = from_alphabet(modulo);
	    output.unshift(letter);
		}
	}

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
fn divide_mode58(input: &mut Vec<u8>) -> u16 {
	let mut remainder = 0;

  for x in input.mut_iter() {
		if *x==0 {continue;}

		let num_base256: u16 = (*x as u16) & 255;
		let temp = remainder * BASE_256 + num_base256;
		*x = (temp / BASE_58) as u8;
		remainder = temp % BASE_58;
	}

	remainder
}

#[cfg(test)]
mod tests {
	mod tests_encode_base58 {
		use super::super::encode_base58;

		#[test]
		fn should_encode_from_string_to_base58_string() {
			// Given
			let azerty = String::from_str("azerty");
	
			// When
			let azerty_encoded = encode_base58(azerty);
	
			// Then
			assert_eq!(azerty_encoded, String::from_str("qYPmmAqv"));
		}
	}

	mod tests_divide_mode58 {
		use super::super::divide_mode58;

		#[test]
		fn should_divide_zero_without_modulo() {
			// Given
			let mut input = vec!(0u8, 0, 0, 0);
	
			// When
			let modulo = divide_mode58(&mut input);
	
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
			let modulo = divide_mode58(&mut input);
	
			// Then
			assert_eq!(*input.get(0), 1);
			assert_eq!(*input.get(1), 186);
			assert_eq!(modulo, 7);
		}
	}

}
