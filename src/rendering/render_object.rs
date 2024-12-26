use crate::{
    handler::handle::Handle,
    shared::{
        square::Square,
        usize2d::{Coord, Usize2d},
    },
};

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
    pub fn write(self, _handle: &dyn Handle, _border: Square) {}
}

#[cfg(test)]
pub mod test {
    use crate::{
        handler::memory_handle::MemoryHandle,
        rendering::sprite::Sprite,
        shared::{
            square::Square,
            usize2d::{Coord, Usize2d},
        },
    };

    use super::RenderObject;

    #[test]
    pub fn write_object() {
        let handle = MemoryHandle::new();
        let top_left = Usize2d::new(3, 4);
        let bottom_right = Usize2d::new(13, 8);
        let square = Square::new(top_left, bottom_right);

        let obj = RenderObject::new(Sprite::default(), Coord::new(10, 6));
        obj.write(&handle, square);
        let actual = handle.get_buffer_content();
        let actual_string = String::from_utf8_lossy(&actual);
        let expected = "X X\n X \nX X"; //TODO: this is in the wrong "location"
        assert_eq!(
            actual_string, expected,
            "Expected:\n{}\nGot:\n{}\n",
            expected, actual_string
        )
    }
}
