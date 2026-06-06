use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum TerminalColors {
    LightGreen = 120,
    Red = 160,
    White = 231,
    Black = 232,
    HotPink = 200,
    #[default]
    Default = -1,
}
impl Display for TerminalColors {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
