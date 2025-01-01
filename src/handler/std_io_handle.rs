use std::{
    fmt::Debug,
    io::{Stdout, Write},
};

use crate::{rendering::colors::TerminalColors, shared::usize2d::Usize2d};

use super::{handle::Handle, handle_error::HandleError};

pub struct StdIOHandle {
    handle: Stdout,
}

impl StdIOHandle {
    pub fn new() -> Self {
        StdIOHandle {
            handle: std::io::stdout(),
        }
    }
}

impl Debug for StdIOHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[StdIOHandle]")
    }
}
impl Write for StdIOHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.handle.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.handle.flush()
    }
}

impl Handle for StdIOHandle {
    fn set_cursor_location(&mut self, coordinate: Usize2d) -> Result<(), HandleError> {
        match write!(self.handle, "\x1b[{};{}H", coordinate.y, coordinate.x) {
            Ok(_) => Ok(()),
            Err(_) => Err(HandleError::SetCursorLocationFailed),
        }
    }
    fn set_foreground_color(&mut self, color: TerminalColors) -> Result<(), HandleError> {
        match write!(self.handle, "\x1b[38;5;{}m", color as u32) {
            Ok(_) => Ok(()),
            Err(_) => Err(HandleError::SetForegroundFailed),
        }
    }
    fn set_background_color(&mut self, color: TerminalColors) -> Result<(), HandleError> {
        match write!(self.handle, "\x1b[38;5;{}m", color as u32) {
            Ok(_) => Ok(()),
            Err(_) => Err(HandleError::SetForegroundFailed),
        }
    }
    fn write_to_location(
        &mut self,
        buf: &[u8],
        coord: crate::shared::usize2d::Coord,
    ) -> Result<usize, HandleError> {
        let _ = self.set_cursor_location(coord)?;
        self.write(buf).map_err(|_| HandleError::WriteFailed)
    }
}
