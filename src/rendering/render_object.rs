use crate::shared::usize2d::{Coord, Usize2d};

use super::sprite::Sprite;

pub struct RenderObject {
    _coord: Usize2d,
    _sprite: Sprite,
}
impl RenderObject {
    pub fn new(sprite: Sprite, location: Coord) -> Self {
        RenderObject {
            _sprite: sprite,
            _coord: location,
        }
    }
}
