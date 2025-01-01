use std::io::Error;

use crate::{
    handler::handle::Handle,
    shared::{
        shared_errors::SharedErrors,
        square::Square,
        usize2d::{Coord, Usize2d},
    },
};

use super::sprite::Sprite;

pub struct RenderObject {
    coordinate: Usize2d,
    sprite: Sprite,
}
impl RenderObject {
    pub fn new(sprite: Sprite, location: Coord) -> Self {
        RenderObject {
            sprite,
            coordinate: location,
        }
    }
    pub fn get_area(&self) -> Square {
        Square::new(
            Usize2d::new(self.coordinate.x, self.coordinate.y),
            Usize2d::new(
                self.coordinate.x + self.sprite.width - 1,
                self.coordinate.y + self.sprite.width - 1,
            ),
        )
    }
    pub fn get_location(&self) -> Usize2d {
        self.coordinate
    }
    pub fn write_clamped(
        &self,
        _handle: &mut dyn Handle,
        _clamp: Square,
    ) -> Result<usize, std::io::Error> {
        // TODO:let to_write = self.sprite.get_content_for_area(clamp, self.coordinate);

        //handle.write(&to_write)
        Err(Error::new(std::io::ErrorKind::Other, "TBI"))
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
