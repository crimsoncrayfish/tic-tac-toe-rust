use super::colors::TerminalColors as TC;

pub struct Sprite {
    name: String,
    width: usize,
    height: usize,
    chars: Vec<Vec<u8>>,
    background_colors: Vec<Vec<TC>>,
    foreground_colors: Vec<Vec<TC>>,
}
impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            name: "Default Sprite".to_string(),
            width: 3,
            height: 3,
            chars: vec![
                vec![b'X', b' ', b'X'],
                vec![b' ', b'X', b' '],
                vec![b'X', b' ', b'X'],
            ],
            background_colors: vec![
                vec![TC::Red, TC::White, TC::Red],
                vec![TC::White, TC::Red, TC::White],
                vec![TC::Red, TC::White, TC::Red],
            ],
            foreground_colors: vec![
                vec![TC::Red, TC::White, TC::Red],
                vec![TC::White, TC::Red, TC::White],
                vec![TC::Red, TC::White, TC::Red],
            ],
        }
    }
}
