use core::fmt::Display;
use windows_sys::Win32::System::Console::{
    CONSOLE_MODE, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT,
};

use super::errors::ConsoleControlErr;

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
impl ConsoleMode {
    pub fn update_mode(current_mode: CONSOLE_MODE, new_mode: Self) -> CONSOLE_MODE {
        let mut mode = current_mode.clone();
        match new_mode {
            ConsoleMode::Cooked => {
                mode |= ENABLE_ECHO_INPUT;
                mode |= ENABLE_LINE_INPUT;
                mode |= ENABLE_PROCESSED_INPUT;
            }
            ConsoleMode::Uncooked => {
                mode &= !ENABLE_ECHO_INPUT;
                mode &= !ENABLE_LINE_INPUT;
                mode &= !ENABLE_PROCESSED_INPUT;
            }
            _ => assert!(
                false,
                "Why are you trying to set the mode to '{}'?",
                new_mode
            ),
        }

        mode
    }

    pub fn match_mode(current_mode: CONSOLE_MODE) -> Result<Self, ConsoleControlErr> {
        match current_mode {
            x if ((x & ENABLE_ECHO_INPUT) != 0
                && (x & ENABLE_PROCESSED_INPUT) != 0
                && (x & ENABLE_LINE_INPUT) != 0) =>
            {
                Ok(ConsoleMode::Cooked)
            }
            x if ((x & ENABLE_ECHO_INPUT) == 0
                && (x & ENABLE_PROCESSED_INPUT) == 0
                && (x & ENABLE_LINE_INPUT) == 0) =>
            {
                Ok(ConsoleMode::Uncooked)
            }
            x if ((x & ENABLE_ECHO_INPUT) == 0
                || (x & ENABLE_PROCESSED_INPUT) == 0
                || (x & ENABLE_LINE_INPUT) == 0) =>
            {
                Ok(ConsoleMode::UncookedPartial)
            }
            _ => {
                return Err(ConsoleControlErr::ModeTypeUnknown);
            }
        }
    }
}
