use crate::shared::usize2d::Usize2d;

use super::sprite::Sprite;

pub struct RenderObject {
    top_left_coord: Usize2d,
    sprite: Sprite,
}
