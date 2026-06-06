use std::fmt::{Display, Formatter};

use crate::{
    assert_shape_match, rendering::colors::TerminalColors as TC, utils::vec_t_writer::write_vec_2d,
    vec_vec_enum_to_string, vec_vec_u8_to_string,
};

use super::usize2d::{Coord, Usize2d};

#[derive(Debug, Clone, PartialEq)]
pub struct PixelGrid {
    chars: Vec<Vec<u8>>,
    background_colors: Vec<Vec<TC>>,
    foreground_colors: Vec<Vec<TC>>,
}
pub type Frame = PixelGrid;

impl Display for PixelGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "[PixelGrid with x {} and y {}]\nchars:\n{}\nfg:\n{}\nbg:\n{}",
            self.width(),
            self.len(),
            vec_vec_u8_to_string!(self.chars),
            vec_vec_enum_to_string!(self.foreground_colors),
            vec_vec_enum_to_string!(self.background_colors)
        )
    }
}

impl Default for PixelGrid {
    fn default() -> Self {
        Self {
            chars: vec![vec![b' '; 1]; 1],
            background_colors: vec![vec![TC::Default; 1]; 1],
            foreground_colors: vec![vec![TC::Default; 1]; 1],
        }
    }
}

impl PixelGrid {
    /// Get a new empty instance of a `PixelGrid`
    ///
    /// # Arguments
    ///
    /// * `width` - the number of columns in the frame
    /// * `heignt` - the number of rows in the frame
    ///
    /// # Returns
    ///
    /// a new grid of pixels with the char, background and foreground colors set to default values
    ///
    /// # Examples
    ///
    /// ```
    /// let pg = PixelGrid::new_empty(10, 15);
    ///
    /// ```
    pub fn default_with_size(cols: usize, rows: usize) -> Self {
        Frame {
            chars: vec![vec![b' '; cols]; rows],
            background_colors: vec![vec![TC::Default; cols]; rows],
            foreground_colors: vec![vec![TC::Default; cols]; rows],
        }
    }
    /// Get a new instance of a `PixelGrid`
    ///
    /// # Arguments
    ///
    /// * `chars` - the characters to be printed out
    /// * `background_colors` - the background colors for the `Frame`
    /// * `foreground_colors` - the foreground color for the `Frame`
    ///
    /// # Returns
    ///
    /// a new frame with the character, background and foreground colors set
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = Frame::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    ///
    /// ```
    pub fn new(
        chars: Vec<Vec<u8>>,
        background_colors: Vec<Vec<TC>>,
        foreground_colors: Vec<Vec<TC>>,
    ) -> Self {
        assert_shape_match!(
            &chars,
            &background_colors,
            "Background colors should match the chars",
        );
        assert_shape_match!(
            &chars,
            &foreground_colors,
            "Background colors should match the chars",
        );
        Frame {
            chars,
            background_colors,
            foreground_colors,
        }
    }
    /// Get the characters that represents the current frame
    ///
    /// # Returns
    ///
    /// the `Vec<Vec<u8>>` of characters that represents the current frame
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = Frame::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// let chars = frame.get_chars();
    ///
    /// ```
    pub fn get_chars(&self) -> &[Vec<u8>] {
        &self.chars
    }
    /// Get the foreground colors that represents the current frame
    ///
    /// # Returns
    ///
    /// the `Vec<Vec<TerminalColors>>` of colors that represents the foreground colors of current frame
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = Frame::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// let chars = frame.get_foreground_colors();
    ///
    /// ```
    pub fn get_foreground_colors(&self) -> &[Vec<TC>] {
        &self.foreground_colors
    }
    /// Get the background colors that represents the current frame
    ///
    /// # Returns
    ///
    /// the `Vec<Vec<TerminalColors>>` of colors that represents the background colors of current frame
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = Frame::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// let chars = frame.get_background_colors();
    ///
    /// ```
    pub fn get_background_colors(&self) -> &[Vec<TC>] {
        &self.background_colors
    }
    /// Get the number of rows for the frame
    ///
    /// # Returns
    ///
    /// the `usize` representing the lenth of the chars `Vec<Vec<u8>>` which should have the same
    /// length as the background and foreground colores
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = Frame::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// let len = frame.len();
    ///
    /// ```
    pub fn len(&self) -> usize {
        self.chars.len()
    }
    /// Check if PixelGrid is empty
    ///
    /// # Returns
    ///
    /// the `bool` representing if the PixelGrid is empty
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = Frame::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// let is_empty = frame.is_empty();
    ///
    /// ```
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
    /// Get the number of cols for the frame
    ///
    /// # Returns
    ///
    /// the `usize` representing the width of the chars `Vec<Vec<u8>>` which should have the same
    /// width as the background and foreground colores
    ///
    /// # Examples
    ///
    /// ```
    /// let pg = PixelGrid::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// let width = frame.width();
    ///
    /// ```
    pub fn width(&self) -> usize {
        self.chars[0].len()
    }

    /// Write one `PixelGrid` to another `PixelGrid` with a coordinate
    ///
    /// # Arguments
    ///
    /// * `other` - the other `PixelGrid`
    /// * `coord` - the starting coordinate for where the new `PixelGrid` should be written to
    ///
    /// # Examples
    ///
    /// ```
    /// let pg = PixelGrid::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// let pg2 = PixelGrid::new(vec![vec![b'b']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// pg.write_subframe(pg2, Coord::default());
    /// ```
    pub fn write_subframe(&mut self, other: PixelGrid, coord: Coord) {
        assert!(self.len() >= other.len() + coord.y,"length of the Pixelgrid being written plus the coordinate should not exceed the current Pixelgrid");
        assert!(self.width() >= other.width() + coord.x,"width of the Pixelgrid being written plus the coordinate should not exceed the current Pixelgrid");
        write_vec_2d(&mut self.chars, other.chars, coord, b' ');
        write_vec_2d(
            &mut self.background_colors,
            other.background_colors,
            coord,
            TC::Default,
        );
        write_vec_2d(
            &mut self.foreground_colors,
            other.foreground_colors,
            coord,
            TC::Default,
        );
    }

    /// Get a subsection of the current `PixelGrid`
    ///
    /// # Arguments
    ///
    /// * `x_start` - the starting x location
    /// * `x_end` - the end x location
    /// * `y_start` - the starting y location
    /// * `y_end` - the ending y location
    ///
    /// # Returns
    ///
    /// A new `PixelGrid` that represents the subsection of the current `PixelGrid` as described by
    /// the coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = Frame::new(vec![vec![b'a']], vec![vec![TerminalColors::Red]], vec![vec![TerminalColors::Blue]]);
    /// let sub_frame = frame.sub_frame(Coord::default(), Coord::new(1,1));
    ///
    /// ```
    pub fn sub_frame(&self, start: Usize2d, end: Usize2d) -> Self {
        assert!(start.x <= end.x);
        assert!(start.y <= end.y);
        assert!(self.len() > start.y);
        assert!(self.len() > end.y);
        assert!(self.width() > end.x);
        assert!(self.width() > end.x);

        PixelGrid::new(
            self.chars[start.y..=end.y]
                .iter()
                .map(|row| row[start.x..=end.x].to_vec())
                .collect(),
            self.background_colors[start.y..=end.y]
                .iter()
                .map(|row| row[start.x..=end.x].to_vec())
                .collect(),
            self.foreground_colors[start.y..=end.y]
                .iter()
                .map(|row| row[start.x..=end.x].to_vec())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default() {
        let actual = Frame::default();

        assert_eq!(actual.chars, vec![vec![b' '; 1]; 1]);
        assert_eq!(actual.background_colors, vec![vec![TC::Default; 1]; 1]);
        assert_eq!(actual.foreground_colors, vec![vec![TC::Default; 1]; 1]);
    }

    #[test]
    fn default_with_size() {
        let cols = 42;
        let rows = 100;
        let actual = Frame::default_with_size(cols, rows);

        assert_eq!(actual.chars, vec![vec![b' '; cols]; rows]);
        assert_eq!(
            actual.background_colors,
            vec![vec![TC::Default; cols]; rows]
        );
        assert_eq!(
            actual.foreground_colors,
            vec![vec![TC::Default; cols]; rows]
        );
    }
    #[test]
    fn new() {
        let chars = vec![vec![b'x']];
        let fc = vec![vec![TC::Black]];
        let bc = vec![vec![TC::Red]];
        let actual = Frame::new(chars.clone(), bc.clone(), fc.clone());
        assert_eq!(*actual.get_chars(), chars);
        assert_eq!(*actual.get_foreground_colors(), fc);
        assert_eq!(*actual.get_background_colors(), bc);
    }
    #[test]
    fn len() {
        let chars = vec![vec![b'x']];
        let fc = vec![vec![TC::Black]];
        let bc = vec![vec![TC::Red]];
        let actual = Frame::new(chars.clone(), bc.clone(), fc.clone());
        assert_eq!(actual.len(), 1);
        let chars = vec![vec![b'x'], vec![b'x']];
        let fc = vec![vec![TC::Black], vec![TC::Black]];
        let bc = vec![vec![TC::Red], vec![TC::Black]];
        let actual = Frame::new(chars.clone(), bc.clone(), fc.clone());
        assert_eq!(actual.len(), 2);
        let chars = vec![vec![b'x', b'x']];
        let fc = vec![vec![TC::Black, TC::Black]];
        let bc = vec![vec![TC::Red, TC::Black]];
        let actual = Frame::new(chars.clone(), bc.clone(), fc.clone());
        assert_eq!(actual.len(), 1);
    }
    #[test]
    fn write_sub_frame() {
        let mut frame = Frame::default_with_size(7, 8);
        let chars = vec![vec![b'x', b'x'], vec![b'x', b'x']];
        let fc = vec![vec![TC::Black, TC::Red], vec![TC::Black, TC::Red]];
        let bc = vec![vec![TC::Red, TC::Black], vec![TC::Red, TC::Black]];
        let to_write = Frame::new(chars.clone(), bc.clone(), fc.clone());
        frame.write_subframe(to_write, Coord::new(3, 5));

        let expected = Frame::new(
            vec![
                "       ".as_bytes().to_vec(),
                "       ".as_bytes().to_vec(),
                "       ".as_bytes().to_vec(),
                "       ".as_bytes().to_vec(),
                "       ".as_bytes().to_vec(),
                "   xx  ".as_bytes().to_vec(),
                "   xx  ".as_bytes().to_vec(),
                "       ".as_bytes().to_vec(),
            ],
            vec![
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Red,
                    TC::Black,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Red,
                    TC::Black,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
            ],
            vec![
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Black,
                    TC::Red,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Black,
                    TC::Red,
                    TC::Default,
                    TC::Default,
                ],
                vec![
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                    TC::Default,
                ],
            ],
        );
        assert_eq!(frame, expected, "Expected:\n{}\nGot:\n{}", expected, frame);
    }
    #[test]
    fn sub_frame() {
        let mut frame = Frame::default_with_size(7, 8);
        let chars = vec![vec![b'x', b'x'], vec![b'x', b'x']];
        let fc = vec![vec![TC::Black, TC::Red], vec![TC::Black, TC::Red]];
        let bc = vec![vec![TC::Red, TC::Black], vec![TC::Red, TC::Black]];
        let to_write = Frame::new(chars.clone(), bc.clone(), fc.clone());
        frame.write_subframe(to_write.clone(), Coord::new(3, 5));

        let test_cases: Vec<(Usize2d, Usize2d, Frame)> = vec![
            (Usize2d::new(3, 5), Usize2d::new(4, 6), to_write),
            (
                Usize2d::new(2, 4),
                Usize2d::new(3, 5),
                Frame::new(
                    vec![vec![b' ', b' '], vec![b' ', b'x']],
                    vec![vec![TC::Default, TC::Default], vec![TC::Default, TC::Red]],
                    vec![vec![TC::Default, TC::Default], vec![TC::Default, TC::Black]],
                ),
            ),
            (
                Usize2d::new(2, 4),
                Usize2d::new(4, 6),
                Frame::new(
                    vec![
                        vec![b' ', b' ', b' '],
                        vec![b' ', b'x', b'x'],
                        vec![b' ', b'x', b'x'],
                    ],
                    vec![
                        vec![TC::Default, TC::Default, TC::Default],
                        vec![TC::Default, TC::Red, TC::Black],
                        vec![TC::Default, TC::Red, TC::Black],
                    ],
                    vec![
                        vec![TC::Default, TC::Default, TC::Default],
                        vec![TC::Default, TC::Black, TC::Red],
                        vec![TC::Default, TC::Black, TC::Red],
                    ],
                ),
            ),
        ];
        for (start, end, expected) in test_cases {
            let sub_frame = frame.sub_frame(start, end);
            assert_eq!(
                sub_frame, expected,
                "Expected:\n{}\nGot:\n{}",
                expected, sub_frame
            );
        }
    }
}
