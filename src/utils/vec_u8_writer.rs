use std::usize;

pub fn write_to_location(original: Vec<u8>, string_to_write: Vec<u8>, index: usize) -> Vec<u8> {
    if original.len() == 0 {
        return string_to_write;
    }

    if original.len() <= index {
        let mut new_vec: Vec<u8> = original.clone();
        pad_vec(&mut new_vec, index);
        new_vec.extend(string_to_write);
        return new_vec;
    }

    overwrite_vec_from_index(original.clone(), string_to_write, index)
}

fn pad_vec(original: &mut Vec<u8>, length: usize) {
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
    fn write_to_empty() {
        let original: Vec<u8> = Vec::new();
        let new = write_to_location(original, "Hello".into(), 0);
        let expected: Vec<u8> = "Hello".into();
        assert_eq!(new, expected)
    }
    #[test]
    fn write_to_not_empty() {
        let original: Vec<u8> = "Hello ".into();
        let new = write_to_location(original.clone(), "World".into(), 6); // Ensure write_to_location takes ownership
        let expected: Vec<u8> = "Hello World".into(); // Note the extra space

        let expected_string = String::from_utf8(expected.clone()); // Clone expected for use as Vec<u8> and String
        assert!(expected_string.is_ok(), "Expected is not valid UTF-8");

        let new_string = String::from_utf8(new.clone());
        assert!(new_string.is_ok(), "New is not valid UTF-8");

        assert_eq!(
            new,
            expected,
            "Got: {:?}, Expected: {:?}",
            new_string.unwrap(),
            expected_string.unwrap()
        );
    }
}
