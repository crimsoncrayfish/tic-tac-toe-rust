use crate::{shared::pixel::Pixel, shared::usize2d::Usize2d};

pub struct Window {
    previous_frame: Vec<Vec<Pixel>>,
    next_frame: Vec<Vec<Pixel>>,
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
    /// let window= Window::init(size);
    /// ```
    fn init(size: Usize2d) -> Self {
        let new_state = vec![vec![Pixel::default(); size.x]; size.y];
        Window {
            previous_frame: new_state.clone(),
            next_frame: new_state.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{pixel::Pixel, usize2d::Usize2d};

    use super::Window;

    #[test]
    fn init() {
        let size = Usize2d::new(10, 20);
        let window = Window::init(size);

        assert_eq!(window.previous_frame, vec![vec![Pixel::default(); 10]; 20]);
        assert_eq!(window.next_frame, vec![vec![Pixel::default(); 10]; 20]);
    }
}
