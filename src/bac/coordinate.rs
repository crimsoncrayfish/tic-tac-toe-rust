use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
