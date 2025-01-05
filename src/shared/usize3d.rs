use std::{fmt::Display, ops::Add};

#[derive(Clone, Copy, Debug, Default)]
pub struct Usize3d {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
pub type Coord3d = Usize3d;
impl Usize3d {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Usize3d { x, y, z }
    }
}
impl Display for Usize3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
impl Add for Usize3d {
    type Output = Usize3d;
    /// Add two Usize3d's together
    ///
    /// # Arguments
    ///
    /// * `rhs` - the `Usize3d` to add to the current one
    ///
    /// # Returns
    ///
    /// A new `Usize3d` that represents the two added together
    ///
    /// # Examples
    ///
    /// ```
    /// let actual = Coord3d::new(10, 20);
    /// let to_add = Coord3d::new(30, 10);
    /// let new = actual.add(to_add);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Usize3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
#[cfg(test)]
mod tests {
    use std::ops::Add;

    use crate::shared::usize3d::{Coord3d, Usize3d};

    #[test]
    fn new() {
        let actual = Usize3d::new(10, 20, 40);
        assert_eq!(actual.x, 10);
        assert_eq!(actual.y, 20);
        assert_eq!(actual.z, 40);
    }
    #[test]
    fn new_coord() {
        let actual = Coord3d::new(10, 20, 50);
        assert_eq!(actual.x, 10);
        assert_eq!(actual.y, 20);
        assert_eq!(actual.z, 50);
    }
    #[test]
    fn add() {
        let actual = Coord3d::new(10, 20, 50);
        let to_add = Coord3d::new(30, 10, 1);
        let new = actual.add(to_add);
        assert_eq!(new.x, 40);
        assert_eq!(new.y, 30);
        assert_eq!(new.z, 51);
    }
}
