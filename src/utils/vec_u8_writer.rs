use std::usize;

/// Write to an existing `Vec<u8>` with a new `Vec<u8>`
///
/// # Arguments
///
/// * `original` - a `Vec<u8>` that needs to be updated
/// * `string_to_write` - a `Vec<u8>` that needs to be inserted
/// * `index` - the starting index where the new string needs to be written to
///
/// # Returns
///
/// A new `Vec<u8>` e.g.
/// 'the original', 'new string', 4 => 'the new string'
///
/// # Examples
///
/// ```
/// let original_vec: Vec<u8> = "original".as_bytes().to_vec();
/// let to_write_vec: Vec<u8> = "new string".as_bytes().to_vec();
/// let result = write_to_location(original_vec.clone(), to_write_vec.clone(), 4);
///
/// ```
///
pub fn write_to_location(original: Vec<u8>, string_to_write: Vec<u8>, index: usize) -> Vec<u8> {
    if original.len() <= index {
        let mut new_vec: Vec<u8> = original.clone();
        pad_vec_up_to(&mut new_vec, index);
        new_vec.extend(string_to_write);
        return new_vec;
    }

    overwrite_vec_from_index(original.clone(), string_to_write, index)
}

fn pad_vec_up_to(original: &mut Vec<u8>, length: usize) {
    while original.len() < length {
        original.push(b' ');
    }
}

fn overwrite_vec_from_index(original: Vec<u8>, string_to_write: Vec<u8>, index: usize) -> Vec<u8> {
    let mut new_vec: Vec<u8> = original[0..index].to_vec();

    let added_len = string_to_write.len();
    new_vec.extend(string_to_write);
    if original.len() > (index + added_len) {
        let end: Vec<u8> = original[index + added_len..original.len()].to_vec();

        new_vec.extend(end);
    }
    new_vec
}

#[cfg(test)]
mod tests {
    use super::write_to_location;

    #[test]
    fn write_to_location_scenarios() {
        let test_cases = vec![
            ("Hello ", "World", 6, "Hello World"),
            ("Rust", " is great", 4, "Rust is great"),
            ("Foo Baz", "Bar", 0, "Bar Baz"),
            ("Foo", "Bar Baz", 0, "Bar Baz"),
            ("12345", "678", 5, "12345678"),
            ("", "Non-empty", 9, "         Non-empty"),
            ("What even", "", 1, "What even"),
            ("the original", "new string", 4, "the new string"),
        ];

        for (i, (original, to_write, location, expected)) in test_cases.iter().enumerate() {
            let original_vec: Vec<u8> = original.as_bytes().to_vec();
            let to_write_vec: Vec<u8> = to_write.as_bytes().to_vec();
            let expected_vec: Vec<u8> = expected.as_bytes().to_vec();

            let result = write_to_location(original_vec.clone(), to_write_vec.clone(), *location);

            let result_string = String::from_utf8(result.clone());
            let expected_string = String::from_utf8(expected_vec.clone());

            assert!(
                expected_string.is_ok(),
                "Test case {}: Expected is not valid UTF-8",
                i
            );
            assert!(
                result_string.is_ok(),
                "Test case {}: Result is not valid UTF-8",
                i
            );

            assert_eq!(
                result,
                expected_vec,
                "Test case {}: Got: {:?}, Expected: {:?}",
                i,
                result_string.unwrap(),
                expected_string.unwrap()
            );
        }
    }
}
