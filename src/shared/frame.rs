use crate::rendering::colors::TerminalColors;

pub type Frame = Vec<Vec<Pixel>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Pixel {
    char: char,
    background_color: TerminalColors,
    foreground_color: TerminalColors,
}
impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            char: ' ',
            background_color: TerminalColors::Default,
            foreground_color: TerminalColors::Default,
        }
    }
}
impl Pixel {
    fn new(char: char, background_color: TerminalColors, foreground_color: TerminalColors) -> Self {
        Pixel {
            char,
            background_color,
            foreground_color,
        }
    }

    pub fn next(self, char: char) -> Self {
        let mut next = self.clone();
        next.char = char;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let actual = Pixel::default();
        assert_eq!(actual.char, ' ');
        assert_eq!(actual.background_color, TerminalColors::Default);
        assert_eq!(actual.foreground_color, TerminalColors::Default);
    }

    #[test]
    fn clone() {
        let actual = Pixel::default();
        let clone = actual.clone();
        assert_eq!(actual.char, clone.char);
        assert_eq!(actual.background_color, clone.background_color);
        assert_eq!(actual.foreground_color, clone.foreground_color);
        assert_eq!(actual, clone);
        assert_ne!(&actual as *const _, &clone as *const _);
    }

    #[test]
    fn new() {
        let actual = Pixel::new('T', TerminalColors::Red, TerminalColors::White);
        assert_eq!(actual.char, 'T');
        assert_eq!(actual.background_color, TerminalColors::Red);
        assert_eq!(actual.foreground_color, TerminalColors::White);
    }

    #[test]
    fn next() {
        let actual = Pixel::new('T', TerminalColors::Red, TerminalColors::White);
        assert_eq!(actual.char, 'T');
        assert_eq!(actual.background_color, TerminalColors::Red);
        assert_eq!(actual.foreground_color, TerminalColors::White);

        let next = actual.next('R');
        assert_eq!(next.char, 'R');
        assert_eq!(next.background_color, TerminalColors::Red);
        assert_eq!(next.foreground_color, TerminalColors::White);
    }
}
