use core::fmt::Display;

pub enum ConsoleMode {
    Cooked,
    UncookedPartial,
    Uncooked,
    None,
}

impl Display for ConsoleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsoleMode::Uncooked => write!(f, "Uncooked"),
            ConsoleMode::UncookedPartial => write!(f, "Partially Uncooked"),
            ConsoleMode::Cooked => write!(f, "Cooked"),
            ConsoleMode::None => write!(f, "NOT FOUND"),
        }
    }
}
