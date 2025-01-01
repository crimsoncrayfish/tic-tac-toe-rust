use std::fmt::Display;

use super::usize2d::{Coord, Usize2d};

#[derive(Clone, Default, Debug)]
/// A struct representing a rectancle in space
///
/// The smallest the rectangle can be is 1x1 if the `top_left` and `bottom_right` coordinates are
/// the same
///
/// # Example
///
/// ```
/// let sqr = Square::new(Usize2d::new(1,1), Usize2d::new(3,4));
/// ```
/// O O O O O
/// O X X X O
/// O X X X O
/// O X X X O
/// O X X X O
/// O O O O O
///
/// The above square starts at the top left corner with coordinate 1,1 and ends at coordinate (3,4)
///
/// (1,1) (2,1) (3,1) (4,1)...
/// (1,2) (2,2) (3,2) (4,2)...
/// (1,3) (2,3) (3,3) (4,3)...
/// ...
pub struct Square {
    top_left: Usize2d,
    bottom_right: Usize2d,

    top_right: Usize2d,
    bottom_left: Usize2d,
}

impl Square {
    /// Create a new instance of the `Square` with the provided coordinates
    ///
    /// # Arguments
    ///
    /// * `top_left` - the coordinate of the top left hand corner of the square
    /// * `bottom_right` - the coordinate of the bottom right hand corner of the square
    ///
    /// # Returns
    /// A boolean value confirming wether the coordinate provided is inside the panel's coordinates
    ///
    /// # Example
    ///
    /// ```
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    ///
    /// let square= Square::new(top_left, bottom_right);
    /// ```
    pub fn new(top_left: Usize2d, bottom_right: Usize2d) -> Self {
        assert!(
            top_left.x <= bottom_right.x,
            "The starting x coordinate ({}) is after the ending x coordinate ({})",
            top_left.x,
            bottom_right.x
        );
        assert!(
            top_left.y <= bottom_right.y,
            "The starting y coordinate ({}) is after the ending y coordinate ({})",
            top_left.y,
            bottom_right.y
        );
        Square {
            top_left: top_left.clone(),
            top_right: Usize2d::new(bottom_right.x, top_left.y),
            bottom_right: bottom_right.clone(),
            bottom_left: Usize2d::new(top_left.x, bottom_right.y),
        }
    }

    /// Test if a coordinate is inside the square
    ///
    /// # Arguments
    ///
    /// * `coord` - the coordinate to be tested
    ///
    /// # Returns
    /// A boolean value confirming wether the coordinate provided is inside the panel's coordinates
    ///
    /// # Example
    ///
    /// ```
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    ///
    /// let square= Square::new(top_left, bottom_right);
    /// let coord = Usize2d::new(8, 7);
    ///
    /// let is_in_square = square.is_in_square(coord);
    /// ```
    pub fn is_in_square(&self, coordinate: Usize2d) -> bool {
        coordinate.x >= self.top_left.x
            && coordinate.x <= self.bottom_right.x
            && coordinate.y >= self.top_left.y
            && coordinate.y <= self.bottom_right.y
    }
    /// Test if a square overlaps with another
    ///
    /// # Arguments
    ///
    /// * `square` - the other square to be tested
    ///
    /// # Returns
    /// A boolean value confirming wether the square overlaps with another
    ///
    /// # Example
    ///
    /// ```
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let square= Square::new(top_left, bottom_right);
    ///
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let square2= Square::new(top_left, bottom_right);
    ///
    /// let overlaps= square.overlaps_with(square2);
    /// ```
    pub fn overlaps_with(&self, other: &Square) -> bool {
        !(other.bottom_right.x < self.top_left.x
            || other.top_left.x > self.bottom_right.x
            || other.bottom_right.y < self.top_left.y
            || other.top_left.y > self.bottom_right.y)
    }

    /// Get the width of the square
    ///
    /// # Returns
    /// a `usize` indicating the width of the square
    ///
    /// # Example
    ///
    /// ```
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let square= Square::new(top_left, bottom_right);
    ///
    /// let width = square.width();
    /// assert!(width == 11);
    /// ```
    pub fn width(&self) -> usize {
        self.bottom_right.x - self.top_left.x + 1
    }
    /// Get the height of the square
    ///
    /// # Returns
    /// a `usize` indicating the height of the square
    ///
    /// # Example
    ///
    /// ```
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let square= Square::new(top_left, bottom_right);
    ///
    /// let height = square.height();
    /// assert!(height == 70);
    /// ```
    pub fn height(&self) -> usize {
        self.bottom_right.y - self.top_left.y + 1
    }
    /// Get the start and end coordinates for the square
    ///
    /// # Returns
    /// a `tuple` that contains the start and end coordinate for the square
    ///
    /// # Example
    ///
    /// ```
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let square= Square::new(top_left, bottom_right);
    ///
    /// let (top_left, bottom_right)= square.get_boundary();
    /// ```
    pub fn get_boundary(&self) -> (Coord, Coord) {
        (self.top_left, self.bottom_right)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Square - (top-left: {}, bottom-right: {})]",
            self.top_left, self.bottom_right
        )
    }
}

#[cfg(test)]
pub mod test {
    use std::panic::catch_unwind;

    use crate::shared::{square::Square, usize2d::Usize2d};

    #[test]
    fn overlap() {
        let test_cases = vec![
            (
                1,
                Square::new(Usize2d::new(0, 0), Usize2d::new(5, 5)),
                Square::new(Usize2d::new(0, 0), Usize2d::new(5, 5)),
                true,
            ),
            (
                2,
                Square::new(Usize2d::new(1, 1), Usize2d::new(5, 5)),
                Square::new(Usize2d::new(0, 0), Usize2d::new(10, 10)),
                true,
            ),
            (
                3,
                Square::new(Usize2d::new(0, 0), Usize2d::new(10, 10)),
                Square::new(Usize2d::new(1, 1), Usize2d::new(5, 5)),
                true,
            ),
            (
                4,
                Square::new(Usize2d::new(3, 5), Usize2d::new(10, 10)),
                Square::new(Usize2d::new(9, 6), Usize2d::new(11, 8)),
                true,
            ),
            (
                5,
                Square::new(Usize2d::new(9, 6), Usize2d::new(11, 8)),
                Square::new(Usize2d::new(3, 5), Usize2d::new(10, 10)),
                true,
            ),
            (
                6,
                Square::new(Usize2d::new(0, 0), Usize2d::new(5, 5)),
                Square::new(Usize2d::new(9, 9), Usize2d::new(10, 10)),
                false,
            ),
            (
                7,
                Square::new(Usize2d::new(9, 9), Usize2d::new(10, 10)),
                Square::new(Usize2d::new(0, 0), Usize2d::new(5, 5)),
                false,
            ),
            (
                8,
                Square::new(Usize2d::new(9, 0), Usize2d::new(10, 0)),
                Square::new(Usize2d::new(0, 0), Usize2d::new(5, 5)),
                false,
            ),
            (
                9,
                Square::new(Usize2d::new(0, 0), Usize2d::new(5, 5)),
                Square::new(Usize2d::new(9, 0), Usize2d::new(10, 0)),
                false,
            ),
            (
                10,
                Square::new(Usize2d::new(1, 9), Usize2d::new(3, 10)),
                Square::new(Usize2d::new(0, 0), Usize2d::new(5, 5)),
                false,
            ),
            (
                11,
                Square::new(Usize2d::new(0, 0), Usize2d::new(5, 5)),
                Square::new(Usize2d::new(1, 9), Usize2d::new(3, 10)),
                false,
            ),
        ];

        for (i, sq1, sq2, expected) in test_cases {
            if expected {
                assert!(
                    sq1.overlaps_with(&sq2),
                    "Test case {}: Square 1 ({}) should overlap with Square 2 ({})",
                    i,
                    sq1,
                    sq2
                )
            } else {
                assert!(
                    !sq1.overlaps_with(&sq2),
                    "Test case {}: Square 1 ({}) should NOT overlap with Square 2 ({})",
                    i,
                    sq1,
                    sq2
                )
            }
        }
    }
    #[test]
    fn valid_coordinate() {
        let top_left = Usize2d::new(5, 7);
        let bottom_right = Usize2d::new(15, 20);

        let too_far_left = Usize2d::new(4, 8);
        let too_far_top = Usize2d::new(7, 6);
        let too_far_right = Usize2d::new(24, 8);
        let too_far_bottom = Usize2d::new(5, 26);

        let just_right = Usize2d::new(12, 16);

        let borderline_top = Usize2d::new(7, 7);
        let borderline_bottom = Usize2d::new(8, 20);
        let borderline_left = Usize2d::new(5, 10);
        let borderline_right = Usize2d::new(15, 10);

        let square = Square::new(top_left, bottom_right);
        let test_cases = vec![
            (too_far_top, false),
            (too_far_left, false),
            (too_far_right, false),
            (too_far_bottom, false),
            (just_right, true),
            (borderline_top, true),
            (borderline_bottom, true),
            (borderline_left, true),
            (borderline_right, true),
        ];

        for (i, (coordinate, expected_result)) in test_cases.iter().enumerate() {
            let result = square.clone().is_in_square(coordinate.clone());
            assert_eq!(
                result, *expected_result,
                "Test case {}: Got: {:?}, Expected: {:?}, With: {} and {}",
                i, result, expected_result, square, coordinate
            );
        }
    }
    #[test]
    fn height_and_width() {
        let expected_width = 15;
        let expected_height = 18;
        let result = Square::new(Usize2d::new(3, 4), Usize2d::new(17, 21));
        let square = result;

        let actual_width = square.width();
        let actual_height = square.height();
        assert!(
            expected_width == actual_width,
            "Width was wrong. Expected: {}, Got: {}, Math: {} - {} + 1",
            expected_width,
            actual_width,
            square.bottom_right.x,
            square.top_left.x,
        );
        assert!(
            expected_height == actual_height,
            "Height was wrong. Expected: {}, Got: {}, Math: {} - {} + 1",
            expected_height,
            actual_height,
            square.bottom_right.y,
            square.top_left.y,
        );
    }

    #[test]
    fn init_fail_left_gt_right() {
        let top_left = Usize2d::new(1003, 4);
        let bottom_right = Usize2d::new(17, 21);

        let result = catch_unwind(|| {
            let _ = Square::new(top_left.clone(), bottom_right.clone());
        });

        assert!(result.is_err(), "Expected panic, but no panic occurred");

        if let Err(err) = result {
            let message = err.downcast_ref::<String>().unwrap();
            assert!(
                message.contains(
                    "The starting x coordinate (1003) is after the ending x coordinate (17)"
                ),
                "Panic message did not match expected format: {}",
                message
            );
        }
    }

    #[test]
    fn init_fail_top_gt_bottom() {
        let top_left = Usize2d::new(2, 20204);
        let bottom_right = Usize2d::new(17, 21);

        let result = catch_unwind(|| {
            let _ = Square::new(top_left.clone(), bottom_right.clone());
        });

        assert!(result.is_err(), "Expected panic, but no panic occurred");

        if let Err(err) = result {
            let message = err.downcast_ref::<String>().unwrap();
            assert!(
                message.contains(
                    "The starting y coordinate (20204) is after the ending y coordinate (21)"
                ),
                "Panic message did not match expected format: {}",
                message
            );
        }
    }
}
