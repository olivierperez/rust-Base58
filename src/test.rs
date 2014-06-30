#[cfg(test)]
mod tests {
  extern crate base58;
  use self::base58::encode_base58;

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

#[cfg(test)]
mod bench {
  extern crate test;
  extern crate base58;

  use self::test::Bencher;
  use self::base58::encode_base58;

  #[bench]
  fn encode_lorem_ispum(b: &mut Bencher) {
    b.iter(|| encode_base58(String::from_str("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")));
  }
}