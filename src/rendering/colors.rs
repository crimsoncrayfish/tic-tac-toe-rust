use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum TerminalColors {
    // --- System Colors (0 - 15) ---
    Black = 0,
    Maroon = 1,
    Green = 2,
    Olive = 3,
    Navy = 4,
    Purple = 5,
    Teal = 6,
    Silver = 7,
    Grey = 8,
    BrightRed = 9,
    BrightGreen = 10,
    Yellow = 11,
    Blue = 12,
    Magenta = 13,
    Cyan = 14,
    BrightWhite = 15,

    LightGreen = 120,
    Red = 160,
    White = 231,
    BlackVGA = 232,
    HotPink = 200,
    Gold = 220,
    Orange = 214,
    DeepSkyBlue = 39,
    ElectricPurple = 129,
    Lime = 46,
    Chartreuse = 118,
    DarkGrey = 240,

    #[default]
    Default = -1,

    ResetFgOnly = -2,
    ResetBgOnly = -3,
    ResetAllStyles = -4,
}
impl Display for TerminalColors {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
