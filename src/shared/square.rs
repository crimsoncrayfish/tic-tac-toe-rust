use std::fmt::Display;

use super::usize2d::Usize2d;

#[derive(Clone, Default, Debug)]
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
    pub fn new(top_left: Usize2d, bottom_right: Usize2d) -> Self {
        Square {
            top_left,
            bottom_right,
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
    pub fn is_in_square(self, coordinate: Usize2d) -> bool {
        coordinate.x >= self.top_left.x
            && coordinate.x <= self.bottom_right.x
            && coordinate.y >= self.top_left.y
            && coordinate.y <= self.bottom_right.y
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
}
