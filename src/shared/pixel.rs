use crate::rendering::colors::TerminalColors;

#[derive(Clone, Debug, PartialEq)]
pub struct Pixel {
    char: char,
    background_color: TerminalColors,
    forground_color: TerminalColors,
}
impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            char: ' ',
            background_color: TerminalColors::Default,
            forground_color: TerminalColors::Default,
        }
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
        assert_eq!(actual.forground_color, TerminalColors::Default);
    }

    #[test]
    fn clone() {
        let actual = Pixel::default();
        let clone = actual.clone();
        assert_eq!(actual.char, clone.char);
        assert_eq!(actual.background_color, clone.background_color);
        assert_eq!(actual.forground_color, clone.forground_color);
        assert_eq!(actual, clone);
        assert_ne!(&actual as *const _, &clone as *const _);
    }
}
