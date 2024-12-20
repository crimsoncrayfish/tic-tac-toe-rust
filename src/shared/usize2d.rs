#[derive(Debug, Default)]
pub struct Usize2d {
    pub x: usize,
    pub y: usize,
}
impl Usize2d {
    pub fn new(x: usize, y: usize) -> Self {
        Usize2d { x, y }
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
