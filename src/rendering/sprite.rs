use super::colors::TerminalColors;

pub struct Sprite {
    name: String,
    width: usize,
    height: usize,
    chars: Vec<Vec<u8>>,
    background_colors: Vec<Vec<TerminalColors>>,
    foreground_colors: Vec<Vec<TerminalColors>>,
}
