use crate::shared::{
    frame::PixelGrid, shared_errors::SharedErrors, square::Square, usize2d::Coord, usize3d::Coord3d,
};

use super::sprite::Sprite;

pub struct RenderObject {
    coordinate: Coord3d,
    sprite: Sprite,
}
impl RenderObject {
    /// Get a new instance of the `RenderObject`
    ///
    /// # Arguments
    ///
    /// * `sprite` - a `Sprite` to be rendered
    /// * `coord` - a coordinate in 3d space. Objects with higher z coordinates will be rendered on
    /// top of objects with smaller z coordinates
    ///
    /// # Returns
    ///
    /// A new `RenderObject`
    ///
    /// # Example
    ///
    /// ```
    /// let object = RenderObject::new(Sprite::default(), Coord3d::default());
    /// ```
    pub fn new(sprite: Sprite, coord: Coord3d) -> Self {
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
    pub fn get_location(&self) -> Coord3d {
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
    pub fn get_content_to_write(&self, clamp: Square) -> Result<PixelGrid, SharedErrors> {
        self.sprite
            .get_content_for_area(self.coordinate.as_2d(), clamp)
    }
}
