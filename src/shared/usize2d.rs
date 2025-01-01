use std::{fmt::Display, ops::Add};

#[derive(Clone, Copy, Debug, Default)]
pub struct Usize2d {
    pub x: usize,
    pub y: usize,
}
pub type Coord = Usize2d;
impl Usize2d {
    pub fn new(x: usize, y: usize) -> Self {
        Usize2d { x, y }
    }
}
impl Display for Usize2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
impl Add for Usize2d {
    type Output = Usize2d;
    fn add(self, rhs: Self) -> Self::Output {
        Usize2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let actual = Usize2d::new(10, 20);
        assert_eq!(actual.x, 10);
        assert_eq!(actual.y, 20);
    }
    #[test]
    fn new_coord() {
        let actual = Coord::new(10, 20);
        assert_eq!(actual.x, 10);
        assert_eq!(actual.y, 20);
    }
}
