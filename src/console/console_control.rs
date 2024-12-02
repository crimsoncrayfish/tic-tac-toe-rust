use std::char;
use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Console::{
    GetConsoleMode, GetStdHandle, ReadConsoleInputA, SetConsoleMode, CONSOLE_MODE, INPUT_RECORD,
    INPUT_RECORD_0, KEY_EVENT_RECORD, KEY_EVENT_RECORD_0, STD_INPUT_HANDLE,
};

use super::errors::ConsoleControlErr;
use super::mode::{self, ConsoleMode};

pub struct ConsoleControl {
    handle: HANDLE,
}
impl ConsoleControl {
    ///Initialize an instance of ConsoleControl
    ///
    /// #Returns
    ///
    /// An a Result that is either an instance of ConsoleControl or an error
    ///
    /// # Examples
    ///
    /// ```
    /// let control= match ConsoleControl::init() {
    ///     Ok(control) => control,
    ///     Err(e) => return e,
    /// };
    /// ```
    pub fn init() -> Result<Self, ConsoleControlErr> {
        let input_handle: HANDLE = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        if input_handle == INVALID_HANDLE_VALUE {
            return Err(ConsoleControlErr::NoHandle);
        }
        Ok(ConsoleControl {
            handle: input_handle,
        })
    }

    /// Get current console mode
    ///
    /// #Returns
    ///
    /// An a Result that is either an enum value representing the current mode
    /// of the console or an error
    ///
    /// Modes:
    /// Cooked - Normel mode
    /// Uncooked - All events need to be processed manually
    ///
    /// See 'set_uncooked_mode' for how to change mode
    ///
    /// # Examples
    ///
    /// ```
    /// let mode = match control.get_console_mode() {
    ///     Ok(mode) => mode,
    ///     Err(e) => return e,
    /// };
    /// ```
    pub fn get_console_mode(&mut self) -> Result<ConsoleMode, ConsoleControlErr> {
        let mode: CONSOLE_MODE = self.get_console_mode_raw()?;
        mode::ConsoleMode::match_mode(mode)
    }

    /// Get current console mode
    ///
    /// #Returns
    ///
    /// An a Result that is either an enum value representing the current mode
    /// of the console or an error
    ///
    /// # Examples
    ///
    /// ```
    /// let mode = match self.get_console_mode() {
    ///     Ok(mode) => mode,
    ///     Err(e) => return e,
    /// };
    /// ```
    fn get_console_mode_raw(&self) -> Result<CONSOLE_MODE, ConsoleControlErr> {
        let mut mode: CONSOLE_MODE = 0;
        let success = unsafe { GetConsoleMode(self.handle, &mut mode) };
        if success == 0 {
            return Err(ConsoleControlErr::NoModeResponse);
        }
        Ok(mode)
    }

    /// Read input from console (requires uncooked mode - see
    ///
    /// #Returns
    ///
    /// An a Result that is either an enum value representing the current mode
    /// of the console or an error
    ///
    /// # Examples
    ///
    /// ```
    /// let mode = match self.get_console_mode() {
    ///     Ok(mode) => mode,
    ///     Err(e) => return e,
    /// };
    /// ```
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

    pub fn set_mode(&self, mode: ConsoleMode) -> Result<(), ConsoleControlErr> {
        let mode = ConsoleMode::update_mode(self.get_console_mode_raw()?, mode);

        let success = unsafe { SetConsoleMode(self.handle, mode) };
        if success == 0 {
            return Err(ConsoleControlErr::NoModeResponse);
        }
        return Ok(());
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
