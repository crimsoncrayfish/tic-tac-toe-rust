use crate::{
    rendering::colors::TerminalColors as TC, utils::vec_vec_helper::assert_vecs_shape_match,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PixelGrid {
    chars: Vec<Vec<u8>>,
    background_colors: Vec<Vec<TC>>,
    foreground_colors: Vec<Vec<TC>>,
}
pub type Frame = PixelGrid;

impl Default for PixelGrid {
    fn default() -> Self {
        Self {
            chars: Vec::new(),
            background_colors: Vec::new(),
            foreground_colors: Vec::new(),
        }
    }
}

impl PixelGrid {
    /// Get a new empty instance of a `Frame`
    ///
    /// # Arguments
    ///
    /// * `width` - the number of columns in the frame
    /// * `heignt` - the number of rows in the frame
    ///
    /// # Returns
    ///
    /// a new frame with the character, background and foreground colors set to default values
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = Frame::new_empty(10, 15);
    ///
    /// ```
    pub fn default_with_size(cols: usize, rows: usize) -> Self {
        Frame {
            chars: vec![vec![b' '; cols]; rows],
            background_colors: vec![vec![TC::Default; cols]; rows],
            foreground_colors: vec![vec![TC::Default; cols]; rows],
        }
    }
    /// Get a new instance of a `Frame`
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
        assert_vecs_shape_match::<u8, TC>(
            &chars,
            &background_colors,
            "Background colors should match the chars",
        );
        assert_vecs_shape_match::<u8, TC>(
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let actual = Frame::default();
        assert_eq!(actual.chars, Vec::<Vec<u8>>::new());
        assert_eq!(actual.background_colors, Vec::<Vec<TC>>::new());
        assert_eq!(actual.foreground_colors, Vec::<Vec<TC>>::new());
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
}
