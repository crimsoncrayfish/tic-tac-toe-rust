use std::fmt::Display;
use winapi::shared::minwindef::{BOOL, DWORD, LPDWORD};
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_INPUT_HANDLE;
use winapi::um::wincon::{ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT};
use winapi::um::winnt::HANDLE;

#[derive(Debug)]
pub enum ConsoleControlErr {
    NoHandle,
    NoModeResponse,
    ModeTypeUnknown,
    SetModeFailed,
}
impl Display for ConsoleControlErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed...")
    }
}

pub struct ConsoleControl {
    handle: HANDLE,
}
impl ConsoleControl {
    pub fn init() -> Result<Self, ConsoleControlErr> {
        let input_handle: HANDLE = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        if input_handle == INVALID_HANDLE_VALUE {
            return Err(ConsoleControlErr::NoHandle);
        }
        Ok(ConsoleControl {
            handle: input_handle,
        })
    }

    pub fn get_console_mode(&mut self) -> Result<ConsoleMode, ConsoleControlErr> {
        let mode: DWORD = self.get_console_mode_raw()?;
        match mode {
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
    fn get_console_mode_raw(&self) -> Result<DWORD, ConsoleControlErr> {
        let mut mode: DWORD = 0;
        let success = unsafe { GetConsoleMode(self.handle, &mut mode as LPDWORD) };
        if success == 0 {
            return Err(ConsoleControlErr::NoModeResponse);
        }
        Ok(mode)
    }

    pub fn set_uncooked_mode(&self) -> Result<(), ConsoleControlErr> {
        let mut mode: DWORD = self.get_console_mode_raw()?;
        mode &= !ENABLE_ECHO_INPUT;
        mode &= !ENABLE_LINE_INPUT;
        mode &= !ENABLE_PROCESSED_INPUT;

        let success = unsafe { SetConsoleMode(self.handle, mode) };
        if success == 0 {
            return Err(ConsoleControlErr::NoModeResponse);
        }
        return Ok(());
    }
    pub fn set_cooked_mode(&self) -> Result<(), ConsoleControlErr> {
        let mut mode: DWORD = self.get_console_mode_raw()?;
        mode |= ENABLE_ECHO_INPUT;
        mode |= ENABLE_LINE_INPUT;
        mode |= ENABLE_PROCESSED_INPUT;

        let success = unsafe { SetConsoleMode(self.handle, mode) };
        if success == 0 {
            return Err(ConsoleControlErr::NoModeResponse);
        }
        return Ok(());
    }
}

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
