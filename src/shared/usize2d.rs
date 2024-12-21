use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct Usize2d {
    pub x: usize,
    pub y: usize,
}
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let actual = Usize2d::new(10, 20);
        assert_eq!(actual.x, 10);
        assert_eq!(actual.y, 20);
    }
}
