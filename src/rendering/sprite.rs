use super::colors::TerminalColors;

pub struct Sprite {
    _name: String,
    _width: usize,
    _height: usize,
    _chars: Vec<Vec<u8>>,
    _background_colors: Vec<Vec<TerminalColors>>,
    _foreground_colors: Vec<Vec<TerminalColors>>,
}
