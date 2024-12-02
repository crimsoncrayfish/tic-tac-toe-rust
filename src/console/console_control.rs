use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Console::{
    GetConsoleMode, GetStdHandle, ReadConsoleInputA, SetConsoleMode, CONSOLE_MODE, INPUT_RECORD,
    STD_INPUT_HANDLE,
};

use super::errors::ConsoleControlErr;
use super::input_record::{EventType, InputRecord, KeyEvent};
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
    ///     Err(e) => return Err(e),
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
    ///     Err(e) => return Err(e),
    /// };
    /// ```
    pub fn get_console_mode(&mut self) -> Result<ConsoleMode, ConsoleControlErr> {
        let mode: CONSOLE_MODE = self.get_console_mode_raw()?;
        mode::ConsoleMode::match_mode(mode)
    }

    /// Get current console mode working directly with the windows APIs
    ///
    /// #Returns
    ///
    /// An a Result that is either an instance of CONSOLE_MODE an error
    ///
    /// # Examples
    ///
    /// ```
    /// let mode = match self.get_console_mode_raw() {
    ///     Ok(mode) => mode,
    ///     Err(e) => return Err(e),
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
    /// An a Result that is either an instance of ConsoleCommand (a wrapper for INPUT_RECORD)
    /// or an error
    ///
    /// # Examples
    ///
    /// ```
    /// let input= match self.read_console_input() {
    ///     Ok(i) => i,
    ///     Err(e) => return Err(e),
    /// };
    /// ```
    pub fn read_console_input(&self) -> Result<KeyEvent, ConsoleControlErr> {
        // TODO: handle these errors
        let input_rec_raw = self.read_console_input_raw()?;
        let input_rec: InputRecord = InputRecord::from_raw(input_rec_raw)?;

        match input_rec.event_type {
            EventType::KeyEvent => Ok(unsafe { input_rec.event.key_event }),
            _ => Err(ConsoleControlErr::WrongEventType),
        }
    }

    /// Read the console raw inputs working directly with the windows APIs
    /// This function waits for the next input.
    ///
    /// #Returns
    ///
    /// An a Result that is either an instance of INPUT_RECORD or an error
    ///
    /// # Examples
    ///
    /// ```
    /// let mode = match self.read_console_input_raw() {
    ///     Ok(mode) => mode,
    ///     Err(e) => return Err(e),
    /// };
    /// ```
    fn read_console_input_raw(&self) -> Result<INPUT_RECORD, ConsoleControlErr> {
        let mut event_count: u32 = 0;
        let mut input_rec: INPUT_RECORD = InputRecord::new_raw();
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
