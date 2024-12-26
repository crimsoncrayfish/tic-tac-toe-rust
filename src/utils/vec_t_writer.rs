use std::usize;

/// Write to an existing `Vec<T>` with a new `Vec<T>` where T is the type
///
/// # Arguments
///
/// * `original` - a `Vec<T>` that needs to be updated
/// * `string_to_write` - a `Vec<T>` that needs to be inserted
/// * `index` - the starting index where the new string needs to be written to
/// * `default` - the value used to pad the vec when writing to a new location
///
/// # Returns
///
/// A new `Vec<T>` e.g.
/// 'the original', 'new string', 4 => 'the new string'
///
/// # Examples
///
/// ```
/// let original_vec: Vec<u8> = "original".as_bytes().to_vec();
/// let to_write_vec: Vec<u8> = "new string".as_bytes().to_vec();
/// let result = write_to_location(original_vec.clone(), to_write_vec.clone(), 4, b' ');
/// assert_eq!(result, "orignew string".as_bytes().to_vec());
/// ```
///
pub fn write_vec_to_vec<T: Copy>(
    original: Vec<T>,
    vec_to_write: Vec<T>,
    index: usize,
    default: T,
) -> Vec<T> {
    let mut new_vec: Vec<T> = Vec::with_capacity(original.len().max(index + vec_to_write.len()));
    if original.len() <= index {
        new_vec.extend(original);
        pad_vec(&mut new_vec, index, default);
        new_vec.extend(vec_to_write);
        return new_vec;
    }
    new_vec.extend_from_slice(&original[0..index]);
    let written_len = vec_to_write.len();
    new_vec.extend(vec_to_write);
    if original.len() > (index + written_len) {
        new_vec.extend_from_slice(&original[index + written_len..]);
    }
    new_vec
}

#[cfg(test)]
mod write_vec_to_vec_tests {
    use crate::{rendering::colors::TerminalColors, utils::vec_t_writer::write_vec_to_vec};

    #[test]
    fn write_u8_to_location_scenarios() {
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

            let result =
                write_vec_to_vec(original_vec.clone(), to_write_vec.clone(), *location, b' ');

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

    #[test]
    fn write_enum_to_location_scenarios() {
        let default = TerminalColors::Black;
        let test_cases = vec![
            (
                vec![TerminalColors::Red, TerminalColors::Black],
                vec![TerminalColors::HotPink],
                4,
                vec![
                    TerminalColors::Red,
                    TerminalColors::Black,
                    default.clone(),
                    default.clone(),
                    TerminalColors::HotPink,
                ],
            ),
            (
                vec![TerminalColors::Red, TerminalColors::Black],
                vec![TerminalColors::HotPink],
                1,
                vec![TerminalColors::Red, TerminalColors::HotPink],
            ),
        ];

        for (i, (original, to_write, location, expected)) in test_cases.iter().enumerate() {
            let result = write_vec_to_vec(
                original.clone(),
                to_write.clone(),
                *location,
                default.clone(),
            );

            assert_eq!(
                &result, expected,
                "Test case {}: Got: {:?}, Expected: {:?}",
                i, result, expected
            );
        }
    }
}
/// Write to an existing `Vec<T>` with a new `Vec<u8>`
///
/// # Arguments
///
/// * `original` - a `Vec<T>` that needs to be updated
/// * `t_to_write` - a value `T` that needs to be inserted
/// * `index` - the starting index where the new value needs to be written to
/// * `len` - the number of instances of the value that needs to be written
/// * `default` - the value used to pad the vec when writing to a new location
///
/// # Returns
///
/// A new `Vec<u8>` e.g.
/// 'the original', b'A', 4, 2, b' ' => 'the AAw string'
///
/// # Examples
///
/// ```
/// let original_vec: Vec<u8> = "original string".as_bytes().to_vec();
/// let to_write: u8 = b"A";
/// let result = write_t_to_vec(original_vec.clone(), to_write, 4,2, b' ');
/// assert_eq!(result, "origAA string".as_bytes().to_vec());
///
/// ```
pub fn write_t_to_vec<T: Copy>(
    original: Vec<T>,
    t_to_write: T,
    index: usize,
    len: usize,
    default: T,
) -> Vec<T> {
    if original.len() <= index {
        let mut new_vec: Vec<T> = original.clone();
        pad_vec(&mut new_vec, index, default);
        pad_vec(&mut new_vec, len + index, t_to_write);
        return new_vec;
    }

    let mut new: Vec<T> = Vec::with_capacity(original.len().max(index + len));
    new.extend_from_slice(&original[0..index]);
    new.extend(std::iter::repeat(t_to_write).take(len));
    if original.len() > len + index {
        new.extend_from_slice(&original[len + index..]);
    }
    new
}
#[cfg(test)]
mod write_t_to_vec_tests {
    use crate::utils::vec_t_writer::write_t_to_vec;

    #[test]
    fn write_u8() {
        let test_cases = vec![
            ("Hello", 6_usize, 4_usize, b'P', b'V', "HelloPVVVV"),
            ("Hello", 2_usize, 1_usize, b'P', b'V', "HeVlo"),
            ("Hello", 2_usize, 10_usize, b'P', b'V', "HeVVVVVVVVVV"),
            ("Hello World", 0_usize, 3_usize, b'P', b' ', "   lo World"),
            ("cat", 5_usize, 4_usize, b' ', b'N', "cat  NNNN"),
            ("cat", 5_usize, 0_usize, b' ', b'N', "cat  "),
        ];

        for (i, (original, index, len, padding_value, t_to_write, expected)) in
            test_cases.iter().enumerate()
        {
            let original_vec: Vec<u8> = original.as_bytes().to_vec();
            let expected_vec: Vec<u8> = expected.as_bytes().to_vec();

            let result: Vec<u8> =
                write_t_to_vec(original_vec, *t_to_write, *index, *len, *padding_value);

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

/// Add padding to a Vec<T> with a specified padding value
///
/// # Arguments
///
/// * `original` - a `Vec<T>` that needs to be updated
/// * `len` - the length that the Vec<T> should be after padding
/// * `default` - the value that should be added in the new entries
///
/// # Returns
///
/// This function modifies the existing Vec<T>
/// 'original', b'A', 10 => 'originalAA'
///
/// # Examples
///
/// ```
/// let original_vec: Vec<u8> = "original string".as_bytes().to_vec();
/// let to_write: u8 = b"A";
/// pad_vec(original_vec, 20, to_write);
/// assert_eq!(original_vec, "original stringAAAAA".as_bytes().to_vec());
///
/// ```
pub fn pad_vec<T: Copy>(original: &mut Vec<T>, len: usize, default: T) {
    if len < original.len() {
        return;
    }
    original.extend(std::iter::repeat(default).take(len - original.len()));
}

#[cfg(test)]
mod pad_vec_tests {
    use crate::utils::vec_t_writer::pad_vec;

    #[test]
    fn pad() {
        let test_cases = vec![
            ("Hello", 6_usize, b' ', "Hello "),
            ("Hello", 3_usize, b' ', "Hello"),
            ("Hello", 10_usize, b'A', "HelloAAAAA"),
        ];

        for (i, (original, new_len, padding_value, expected)) in test_cases.iter().enumerate() {
            let original_vec: Vec<u8> = original.as_bytes().to_vec();
            let expected_vec: Vec<u8> = expected.as_bytes().to_vec();

            let mut result = original_vec.clone();
            pad_vec(&mut result, *new_len, *padding_value);

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
