use crate::{
    assert_r,
    shared::{frame::Pixel, usize2d::Usize2d},
};

use super::errors::WindowException;

#[derive(Debug)]
pub struct Window {
    previous_frame: Vec<Vec<Pixel>>,
    next_frame: Vec<Vec<Pixel>>,
    top_left: Usize2d,
    bottom_right: Usize2d,
}
impl Window {
    ///Initialize an instance of Window
    ///
    /// # Arguments
    ///
    /// * `size` - number of rows and columns in the grid as a usize
    ///
    /// #Returns
    ///
    /// An instance of a window
    ///
    /// # Examples
    ///
    /// ```
    /// let size = Usize2d::new(10, 69);
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let window= Window::init(size, top_left, bottom_right);
    /// ```
    fn init(
        size: Usize2d,
        top_left: Usize2d,
        bottom_right: Usize2d,
    ) -> Result<Self, WindowException> {
        let new_state = vec![vec![Pixel::default(); size.x]; size.y];
        assert_r!(
            top_left.x < bottom_right.x,
            WindowException::BadCoordinateException
        );
        assert_r!(
            top_left.y < bottom_right.y,
            WindowException::BadCoordinateException
        );
        Ok(Window {
            previous_frame: new_state.clone(),
            next_frame: new_state.clone(),
            top_left,
            bottom_right,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shared::{frame::Pixel, usize2d::Usize2d},
        windows::errors::WindowException,
    };

    use super::Window;

    #[test]
    fn init() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let window = Window::init(size, top_left, bottom_right);
        assert!(window.is_ok());
        let window = window.unwrap();

        assert_eq!(window.previous_frame, vec![vec![Pixel::default(); 10]; 20]);
        assert_eq!(window.next_frame, vec![vec![Pixel::default(); 10]; 20]);
    }

    #[test]
    fn init_fail() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(1, 0);
        let bottom_right = Usize2d::new(0, 20);
        let window = Window::init(size, top_left, bottom_right);
        assert!(window.is_err());
        let error = window.unwrap_err();
        assert!(error == WindowException::BadCoordinateException);
    }
}
