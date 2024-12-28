use std::fmt::Display;

use crate::assert_r;

use super::{shared_errors::SharedErrors, usize2d::Usize2d};

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
    pub fn new(top_left: Usize2d, bottom_right: Usize2d) -> Result<Self, SharedErrors> {
        assert_r!(top_left.x <= bottom_right.x, SharedErrors::BadCoordinate);
        assert_r!(top_left.y <= bottom_right.y, SharedErrors::BadCoordinate);
        Ok(Square {
            top_left,
            bottom_right,
        })
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
    pub fn is_in_square(self, coordinate: Usize2d) -> bool {
        coordinate.x >= self.top_left.x
            && coordinate.x <= self.bottom_right.x
            && coordinate.y >= self.top_left.y
            && coordinate.y <= self.bottom_right.y
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
    use crate::shared::{square::Square, usize2d::Usize2d};

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

        let square = Square {
            top_left,
            bottom_right,
        };
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
        assert!(result.is_ok());
        let square = result.unwrap();

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
        let result = Square::new(top_left.clone(), bottom_right.clone());
        assert!(result.is_err(), "The left most coordinate ({}), cannot be more to the right than the right most coordinate ({})",top_left.x, bottom_right.x);
    }

    #[test]
    fn init_fail_top_gt_bottom() {
        let top_left = Usize2d::new(2, 20204);
        let bottom_right = Usize2d::new(17, 21);
        let result = Square::new(top_left.clone(), bottom_right.clone());
        assert!(result.is_err(), "The top most coordinate ({}), cannot be more to the bottom than the bottom most coordinate ({})",top_left.y, bottom_right.y);
    }
}
