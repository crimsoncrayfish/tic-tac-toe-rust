use std::io::Error;

use crate::{
    handler::handle::Handle,
    shared::{shared_errors::SharedErrors, square::Square, usize2d::Coord},
};

use super::sprite::Sprite;

pub struct RenderObject {
    coordinate: Coord,
    sprite: Sprite,
}
impl RenderObject {
    pub fn new(sprite: Sprite, coord: Coord) -> Self {
        RenderObject {
            sprite,
            coordinate: coord,
        }
    }
    pub fn get_area(&self) -> Square {
        Square::new(
            Coord::new(self.coordinate.x, self.coordinate.y),
            Coord::new(
                self.coordinate.x + self.sprite.width - 1,
                self.coordinate.y + self.sprite.width - 1,
            ),
        )
    }
    pub fn get_location(&self) -> Coord {
        self.coordinate
    }

    /// Get the content that should be written to the screen given the limitations i.t.o
    /// coordinates and available screen space
    ///
    /// # Arguments
    ///
    /// * `clamp` - a `Square` describing the available screen space
    ///
    /// # Returns
    ///
    /// A `Vec<Vec<u8>>` of the content to be written
    ///
    /// # Example
    ///
    /// ```
    /// let clamp = Square::default();
    /// let content_to_write: Vec<Vec<u8>> = render_object.get_content_to_write(clamp);
    /// ```
    pub fn get_content_to_write(&self, clamp: Square) -> Result<Vec<Vec<u8>>, SharedErrors> {
        self.sprite.get_content_for_area(self.coordinate, clamp)
    }
}
