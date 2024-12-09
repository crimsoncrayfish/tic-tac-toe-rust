use core::fmt::Display;
#[derive(PartialEq)]
pub enum Command {
    QUIT,
    RESET,
    PAUSEPLAY,
    TOGGLEMODE,
    TOGGLEFPS,
    MOVELEFT,
    MOVERIGHT,
    MOVEUP,
    MOVEDOWN,
    NOMAPPING,
    NONE,
}
impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::QUIT => write!(f, "Quit"),
            Command::RESET => write!(f, "Reset"),
            Command::PAUSEPLAY => write!(f, "Toggle pause"),
            Command::TOGGLEMODE => write!(f, "Toggle print mode"),
            Command::TOGGLEFPS => write!(f, "Toggle fps"),
            Command::MOVELEFT => write!(f, "Move the board left"),
            Command::MOVERIGHT => write!(f, "Move the board right"),
            Command::MOVEUP => write!(f, "Move the board up"),
            Command::MOVEDOWN => write!(f, "Move the board down"),
            Command::NOMAPPING => write!(f, "Key not mapped"),
            Command::NONE => write!(f, "NONE"),
        }
    }
}
