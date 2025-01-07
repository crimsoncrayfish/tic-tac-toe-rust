use crate::{shared::usize2d::Coord, utils::vec_t_writer::write_vec};

pub fn assert_vecs_shape_match<T1, T2>(vec_1: &Vec<Vec<T1>>, vec_2: &Vec<Vec<T2>>, message: &str) {
    assert!(vec_1.len() == vec_2.len(), "{}", message);
    for index in 0..vec_1.len() {
        assert!(vec_1[index].len() == vec_2[index].len(), "{}", message);
    }
}

#[cfg(test)]
mod shape_match_test {
    use std::panic;

    use crate::rendering::colors::TerminalColors;

    use super::assert_vecs_shape_match;

    #[test]
    fn matched() {
        let chars = vec![vec![b'x']];
        let fc = vec![vec![TerminalColors::Black]];
        let bc = vec![vec![TerminalColors::Red]];

        assert_vecs_shape_match(&chars, &fc, "Chars and foreground colors should match");
        assert_vecs_shape_match(&chars, &bc, "Chars and background colors should match");
        assert_vecs_shape_match(&fc, &bc, "Chars and foreground colors should match");

        let chars = vec![vec![b'x'], vec![b'y']];
        let fc = vec![vec![TerminalColors::Black], vec![TerminalColors::Red]];
        let bc = vec![vec![TerminalColors::Red], vec![TerminalColors::HotPink]];

        assert_vecs_shape_match(&chars, &fc, "Chars and foreground colors should match");
        assert_vecs_shape_match(&chars, &bc, "Chars and background colors should match");
        assert_vecs_shape_match(&fc, &bc, "Chars and foreground colors should match");
    }

    #[test]
    fn not_matched() {
        let chars = vec![vec![b'x']];
        let fc = vec![vec![TerminalColors::Black], vec![TerminalColors::Red]];
        let bc: Vec<Vec<TerminalColors>> = vec![];

        let result = panic::catch_unwind(|| {
            assert_vecs_shape_match(&chars, &fc, "Chars and foreground colors should match")
        });
        assert!(result.is_err(), "Expected a panic due to mismatched shapes");
        let result = panic::catch_unwind(|| {
            assert_vecs_shape_match(&chars, &bc, "Chars and background colors should match");
        });
        assert!(result.is_err(), "Expected a panic due to mismatched shapes");
        let result = panic::catch_unwind(|| {
            assert_vecs_shape_match(&fc, &bc, "Foreground and background colors should match");
        });
        assert!(result.is_err(), "Expected a panic due to mismatched shapes");
    }
}
