use std::usize;

use crate::{
    assert_r,
    shared::{shared_errors::SharedErrors, square::Square, usize2d::Coord},
};

use super::colors::TerminalColors as TC;

pub struct Sprite {
    _name: String,
    pub width: usize,
    pub height: usize,
    chars: Vec<Vec<u8>>,
    _background_colors: Vec<Vec<TC>>,
    _foreground_colors: Vec<Vec<TC>>,
}
impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            _name: "Default Sprite".to_string(),
            width: 3,
            height: 3,
            chars: vec![
                vec![b'X', b' ', b'X'],
                vec![b' ', b'X', b' '],
                vec![b'X', b' ', b'X'],
            ],
            _background_colors: vec![
                vec![TC::Red, TC::White, TC::Red],
                vec![TC::White, TC::Red, TC::White],
                vec![TC::Red, TC::White, TC::Red],
            ],
            _foreground_colors: vec![
                vec![TC::Red, TC::White, TC::Red],
                vec![TC::White, TC::Red, TC::White],
                vec![TC::Red, TC::White, TC::Red],
            ],
        }
    }
}
impl Sprite {
    /// Create a new instance of a sprite
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sprite
    /// * `width` - The total number of characters that the sprite is wide
    /// * `height` - The total number of characters that the sprite is tall
    /// * `chars` - The characters that makes up the sprite as a `Vec<Vec<u8>>`
    /// * `background_colors` - The colors that makes up the sprite's background as a
    /// `Vec<Vec<TerminalColors>>`
    /// * `foreground_colors` - The colors that makes up the sprite's foreground as a
    /// `Vec<Vec<TerminalColors>>`
    ///
    /// # Result
    ///
    /// The sprite
    ///
    /// # Example
    ///
    pub fn new(
        name: String,
        width: usize,
        height: usize,
        chars: Vec<Vec<u8>>,
        background_colors: Vec<Vec<TC>>,
        foreground_colors: Vec<Vec<TC>>,
    ) -> Self {
        assert_eq!(
            chars.len(),
            height,
            "The height of the sprite is wrong. Length read: {}, Length in metadata: {}",
            chars.len(),
            height
        );
        for index in 0..chars.len() {
            assert_eq!(
                chars[index].len(),
                width,
                "The width of the sprite on row {} is wrong. Width read: {}, Width in metadata: {}",
                index,
                chars[index].len(),
                width
            );
        }
        Sprite {
            _name: name,
            width,
            height,
            chars,
            _background_colors: background_colors,
            _foreground_colors: foreground_colors,
        }
    }
    ///
    /// Get the sprite content to be rendered given a coordinate and a space to write to
    ///
    /// # Arguments
    ///
    /// * `coord` - an object to be rendered
    /// * `area` - the available area that can be written to
    ///
    /// # Result
    ///
    /// The content of the sprite that should be written after it has been "clamped"
    ///
    /// # Example
    ///
    /// for an area with a starting coordinate of (0, 0) and an ending coordinate of (2, 2)
    /// and the following sprite with a location of 1,2
    /// XYZ
    /// ABC
    /// 123
    ///
    /// The following output can be expected:
    /// |   |
    /// |   |
    /// | XY|
    pub fn get_content_for_area(
        &self,
        coord: Coord,
        area: Square,
    ) -> Result<Vec<Vec<u8>>, SharedErrors> {
        let (area_top_left, area_bottom_right) = area.get_boundary();

        let (x_start, x_end) = Self::get_indexes_in_range(
            coord.x,
            coord.x + self.width - 1,
            area_top_left.x,
            area_bottom_right.x,
        )?;

        let (y_start, y_end) = Self::get_indexes_in_range(
            coord.y,
            coord.y + self.height - 1,
            area_top_left.y,
            area_bottom_right.y,
        )?;

        Ok(self.chars[y_start..=y_end]
            .iter()
            .map(|row| row[x_start..=x_end].to_vec())
            .collect())
    }

    /// Helper function to get the start and end indexes in a range
    ///
    /// # Arguments
    ///
    /// * `coord_start` - the starting coordinate to test
    /// * `coord_end` - the ending coordinate to test
    /// * `range_start` - the starting coordinate of the range to test
    /// * `range_end` - the ending coordinate of the range to test
    /// * `max_index` - the maximum index that can be returned
    ///
    /// #Returns
    ///
    /// if `OK` returns a tuple with the starting and ending index as a usize
    /// if `Err` returns `SharedErrors::OutOfBounds` to indicate that there is no overlap
    ///
    /// # Examples
    ///
    /// ```
    /// let index = get_index_in_range(4, 5,1,5);
    /// assert!(index == 3, "The index of pos 4 should be 3 in the range (1 to 5)");
    /// ```
    fn get_indexes_in_range(
        coord_start: usize,
        coord_end: usize,
        range_start: usize,
        range_end: usize,
    ) -> Result<(usize, usize), SharedErrors> {
        assert!(
            coord_start < coord_end,
            "The starting coordinate ({}) may not be before the ending coordinate ({})",
            coord_start,
            coord_end
        );
        assert!(
            range_start < range_end,
            "The starting range coordinate ({}) may not be before the ending range coordinate ({})",
            range_start,
            range_end
        );
        assert_r!(coord_start < range_end, SharedErrors::OutOfBounds);
        assert_r!(coord_end > range_start, SharedErrors::OutOfBounds);
        let starting_index = match Self::get_index_in_range(range_start, coord_start, coord_end) {
            Ok(v) => v,
            Err(SharedErrors::BeforeStart) => 0,
            Err(SharedErrors::AfterEnd) => return Err(SharedErrors::OutOfBounds),
            Err(e) => return Err(e),
        };
        let ending_index = match Self::get_index_in_range(range_end, coord_start, coord_end) {
            Ok(v) => v,
            Err(SharedErrors::BeforeStart) => return Err(SharedErrors::OutOfBounds),
            Err(SharedErrors::AfterEnd) => coord_end - coord_start,
            Err(e) => return Err(e),
        };

        assert!(
            starting_index < ending_index,
            "Invalid indexes. The starting index ({}) is after the ending index ({}).",
            starting_index,
            ending_index
        );
        Ok((starting_index, ending_index))
    }

    /// Helper function to get the index of a coordinate in a range
    ///
    /// # Arguments
    ///
    /// * `coord` - the coordinate to be tested
    /// * `start` - the starting coordinate of the range
    /// * `end` - the ending coordinate of the range
    ///
    /// #Returns
    ///
    /// An index as a `usize`
    ///
    /// # Examples
    ///
    /// ```
    /// let index = get_index_in_range(4,1,5);
    /// assert!(index == 3, "The index of pos 4 should be 3 in the range (1 to 5)");
    /// ```
    fn get_index_in_range(coord: usize, start: usize, end: usize) -> Result<usize, SharedErrors> {
        assert_r!(coord > start, SharedErrors::BeforeStart);
        assert_r!(coord < end, SharedErrors::AfterEnd);
        Ok(coord - start)
    }
}

#[cfg(test)]
pub mod test {

    use crate::{
        rendering::colors::TerminalColors as TC,
        shared::{shared_errors::SharedErrors, square::Square, usize2d::Coord},
        vec_vec_u8_to_string,
    };

    use super::Sprite;

    #[test]
    fn get_content_to_write() {
        let content: Vec<Vec<u8>> = vec![
            vec![b'x', b'a', b'1'], //
            vec![b'y', b'b', b'2'], //
            vec![b'z', b'c', b'3'], //
            vec![b'0', b'0', b'0'], //
        ];
        let background_colors: Vec<Vec<TC>> = vec![
            vec![TC::Red, TC::Black, TC::LightGreen], //
            vec![TC::Red, TC::Black, TC::LightGreen], //
            vec![TC::Red, TC::Black, TC::LightGreen], //
            vec![TC::Red, TC::Black, TC::LightGreen], //
        ];
        let foreground_colors: Vec<Vec<TC>> = vec![
            vec![TC::Red, TC::Black, TC::LightGreen], //
            vec![TC::Red, TC::Black, TC::LightGreen], //
            vec![TC::Red, TC::Black, TC::LightGreen], //
            vec![TC::Red, TC::Black, TC::LightGreen], //
        ];
        let sprite = Sprite::new(
            "Test Sprite".to_string(),
            content[0].len(),
            content.len(),
            content,
            background_colors,
            foreground_colors,
        );

        let expected: Vec<Vec<u8>> = vec![
            vec![b'x', b'a'], //
            vec![b'y', b'b'], //
            vec![b'z', b'c'], //
        ];
        let actual_result = sprite.get_content_for_area(
            Coord::default(),
            Square::new(Coord::default(), Coord::new(1, 2)),
        );
        assert!(
            actual_result.is_ok(),
            "There should be no error while getting the content to write. Got error {}",
            actual_result.unwrap_err()
        );
        let actual = actual_result.unwrap();
        assert_eq!(
            expected,
            actual,
            "Expected:\n{}\nGot:\n{}",
            vec_vec_u8_to_string!(expected),
            vec_vec_u8_to_string!(actual)
        );
    }
    #[test]
    fn get_index() {
        let test_cases = vec![
            (1, 0, 5, 1, true, SharedErrors::None),
            (3, 1, 5, 2, true, SharedErrors::None),
            (0, 1, 5, 0, false, SharedErrors::BeforeStart),
            (11, 1, 5, 0, false, SharedErrors::AfterEnd),
        ];
        for (coord, range_start, range_end, expected_index, is_in_range, expected_error) in
            test_cases
        {
            let index = Sprite::get_index_in_range(coord, range_start, range_end);
            if is_in_range {
                assert!(
                    index.is_ok(),
                    "The coordinate should be in the expected range"
                );
                let actual = index.unwrap();

                assert_eq!(
                    actual, expected_index,
                    "The index of coordinate {}, in the range {} to {} should be {}, got {}",
                    coord, range_start, range_end, expected_index, actual
                );
            } else {
                assert!(
                    index.is_err(),
                    "The coordinate should be outside of the expected range"
                );
                let actual = index.unwrap_err();
                assert_eq!(
                    actual, expected_error,
                    "Expected error {}, Got error {}",
                    expected_error, actual
                );
            }
        }
    }
    #[test]
    fn test_get_indexes() {
        let test_cases = vec![
            ((1, 8), (0, 5), (0, 4), true, SharedErrors::None), //leans right
            ((1, 8), (6, 12), (5, 7), true, SharedErrors::None), //leans left
            ((3, 8), (0, 12), (0, 5), true, SharedErrors::None), //inside
            ((3, 8), (4, 6), (1, 3), true, SharedErrors::None), //over
            ((1, 3), (5, 10), (0, 0), false, SharedErrors::OutOfBounds), //oob
            ((11, 13), (5, 10), (0, 0), false, SharedErrors::OutOfBounds), //oob
        ];

        for (
            (coord_start, coord_end),
            (range_start, range_end),
            (expected_start, expected_end),
            is_in_range,
            expected_error,
        ) in test_cases
        {
            let index =
                Sprite::get_indexes_in_range(coord_start, coord_end, range_start, range_end);

            if is_in_range {
                assert!(
                    index.is_ok(),
                    "The coordinates should be in the expected range, but got error: {:?}",
                    index
                );
                let (actual_start, actual_end) = index.unwrap();
                assert_eq!(
                    actual_start, expected_start,
                    "The start coordinate should be {}, but got {}",
                    expected_start, actual_start
                );
                assert_eq!(
                    actual_end, expected_end,
                    "The end coordinate should be {}, but got {}",
                    expected_end, actual_end
                );
            } else {
                assert!(
                    index.is_err(),
                    "The coordinates should be outside of the expected range, but got result: {:?}",
                    index
                );
                let actual = index.unwrap_err();
                assert_eq!(
                    actual, expected_error,
                    "Expected error {:?}, but got error {:?}",
                    expected_error, actual
                );
            }
        }
    }
}
