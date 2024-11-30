use std::char;
use std::fmt::Display;
use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Console::{
    GetConsoleMode, GetStdHandle, ReadConsoleInputA, SetConsoleMode, CONSOLE_MODE,
    ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT, INPUT_RECORD, INPUT_RECORD_0,
    KEY_EVENT_RECORD, KEY_EVENT_RECORD_0, STD_INPUT_HANDLE,
};

#[derive(Debug)]
pub enum ConsoleControlErr {
    NoHandle,
    NoModeResponse,
    ModeTypeUnknown,
    SetModeFailed,
    NoInputRead,
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
        let mode: CONSOLE_MODE = self.get_console_mode_raw()?;
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

    fn get_console_mode_raw(&self) -> Result<CONSOLE_MODE, ConsoleControlErr> {
        let mut mode: CONSOLE_MODE = 0;
        let success = unsafe { GetConsoleMode(self.handle, &mut mode) };
        if success == 0 {
            return Err(ConsoleControlErr::NoModeResponse);
        }
        Ok(mode)
    }

    pub fn read_console_input(&self) -> Result<ConsoleCommand, ConsoleControlErr> {
        let input_rec: INPUT_RECORD = self.read_console_input_raw()?;

        //todo: figure our this union thing
        let cmd = ConsoleCommand {
            command: char::from_u32(unsafe { input_rec.Event.KeyEvent.uChar.UnicodeChar as u32 })
                .unwrap(),
            repreat_count: unsafe { input_rec.Event.KeyEvent.wRepeatCount },
            is_down: unsafe { input_rec.Event.KeyEvent.bKeyDown } == 1,
        };
        Ok(cmd)
    }

    fn read_console_input_raw(&self) -> Result<INPUT_RECORD, ConsoleControlErr> {
        let mut event_count: u32 = 0;
        let mut input_rec: INPUT_RECORD = new_input_rec();
        let success =
            unsafe { ReadConsoleInputA(self.handle, &mut input_rec, 1, &mut event_count) };
        if success == 0 {
            return Err(ConsoleControlErr::NoInputRead);
        }
        Ok(input_rec)
    }

    pub fn set_uncooked_mode(&self) -> Result<(), ConsoleControlErr> {
        let mut mode = self.get_console_mode_raw()?;
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
        let mut mode = self.get_console_mode_raw()?;
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

fn new_input_rec() -> INPUT_RECORD {
    let key_event_rec = KEY_EVENT_RECORD_0 { UnicodeChar: 0 };
    let key_event = KEY_EVENT_RECORD {
        bKeyDown: 0,
        wRepeatCount: 0,
        wVirtualKeyCode: 0,
        wVirtualScanCode: 0,
        uChar: key_event_rec,
        dwControlKeyState: 0,
    };
    let event = INPUT_RECORD_0 {
        KeyEvent: key_event,
    };
    INPUT_RECORD {
        EventType: 0,
        Event: event,
    }
}
#[derive(Clone)]
pub struct ConsoleCommand {
    pub command: char,
    pub repreat_count: u16,
    pub is_down: bool,
}
