/// macro to assert that 2d vecs have a similar shape i.e. for each row it has the same length
#[macro_export]
macro_rules! assert_shape_match {
    ($left:expr, $right:expr $(,)?) => {
        $crate::assert_shape_match!($left, $right, "Vectors do not have matching shape");
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        let left_val = &$left;
        let right_val = &$right;
        assert_eq!(
            left_val.len(),
            right_val.len(),
            "Outer vector length mismatch: {} vs {}\nContext: {}",
            left_val.len(),
            right_val.len(),
            format_args!($($arg)+)
        );
        for (i, (left_row, right_row)) in left_val.iter().zip(right_val.iter()).enumerate() {
            assert_eq!(
                left_row.len(),
                right_row.len(),
                "Row {} length mismatch: {} vs {}\nContext: {}",
                i,
                left_row.len(),
                right_row.len(),
                format_args!($($arg)+)
            );
        }
    };
}
#[cfg(test)]
mod shape_match_test {
    use std::panic;

    use crate::rendering::colors::TerminalColors;

    #[test]
    fn matched() {
        let chars = vec![vec![b'x']];
        let fc = vec![vec![TerminalColors::Black]];
        let bc = vec![vec![TerminalColors::Red]];

        assert_shape_match!(&chars, &fc, "Chars and foreground colors should match");
        assert_shape_match!(&chars, &bc, "Chars and background colors should match");
        assert_shape_match!(&fc, &bc, "Chars and foreground colors should match");

        let chars = vec![vec![b'x'], vec![b'y']];
        let fc = vec![vec![TerminalColors::Black], vec![TerminalColors::Red]];
        let bc = vec![vec![TerminalColors::Red], vec![TerminalColors::HotPink]];

        assert_shape_match!(&chars, &fc, "Chars and foreground colors should match");
        assert_shape_match!(&chars, &bc, "Chars and background colors should match");
        assert_shape_match!(&fc, &bc, "Chars and foreground colors should match");
    }

    #[test]
    fn not_matched() {
        let chars = vec![vec![b'x']];
        let fc = vec![vec![TerminalColors::Black], vec![TerminalColors::Red]];
        let bc: Vec<Vec<TerminalColors>> = vec![];
        let result = panic::catch_unwind(|| {
            assert_shape_match!(&chars, &bc, "Chars and background colors should match");
        });
        assert!(result.is_err(), "Expected a panic due to mismatched shapes");
        let result = panic::catch_unwind(|| {
            assert_shape_match!(&fc, &bc, "Foreground and background colors should match");
        });
        assert!(result.is_err(), "Expected a panic due to mismatched shapes");

        let result = panic::catch_unwind(|| {
            assert_shape_match!(&chars, &fc, "Chars and foreground colors should match");
        });
        assert!(result.is_err(), "Expected a panic due to mismatched shapes");
    }
}
