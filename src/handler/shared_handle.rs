use std::{
    error::Error,
    fmt::Display,
    io::{self, Write},
    sync::{Arc, Mutex},
};

use super::{handle::Handle, handle_error::HandleError};

#[derive(Debug)]
pub struct SharedHandle<H: Handle> {
    handle: Arc<Mutex<Box<H>>>,
}

impl<H: Handle> SharedHandle<H> {
    pub fn init(writer: Arc<Mutex<Box<H>>>) -> Self {
        SharedHandle { handle: writer }
    }
    pub fn write(&self, args: std::fmt::Arguments) -> Result<(), SharedWriterErr> {
        let mut locked_writer = match self.handle.lock() {
            Ok(result) => result,
            Err(_) => return Err(SharedWriterErr::FailedToLock),
        };
        let _ = locked_writer.write_fmt(args);
        Ok(())
    }
    pub fn writeln(&self, args: std::fmt::Arguments) -> Result<(), SharedWriterErr> {
        let mut locked_writer = match self.handle.lock() {
            Ok(result) => result,
            Err(_) => return Err(SharedWriterErr::FailedToLock),
        };
        let _ = locked_writer.write_fmt(args);
        let _ = locked_writer.write_all(b"\n");
        Ok(())
    }
    pub fn flush(&self) -> Result<(), SharedWriterErr> {
        let mut locked_writer = match self.handle.lock() {
            Ok(result) => result,
            Err(_) => return Err(SharedWriterErr::FailedToLock),
        };
        let _ = locked_writer.flush();
        Ok(())
    }
}
impl<H: Handle> Write for SharedHandle<H> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut locked_writer = self
            .handle
            .lock()
            .map_err(|_| SharedWriterErr::FailedToLock)?;
        locked_writer.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        let mut locked_writer = self
            .handle
            .lock()
            .map_err(|_| SharedWriterErr::FailedToLock)?;
        locked_writer.flush()
    }
    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()> {
        let mut locked_writer = self
            .handle
            .lock()
            .map_err(|_| SharedWriterErr::FailedToLock)?;
        locked_writer.write_fmt(fmt)
    }
}
impl<H: Handle> Handle for SharedHandle<H> {
    fn set_cursor_location(
        &mut self,
        coord: crate::shared::usize2d::Usize2d,
    ) -> Result<(), super::handle_error::HandleError> {
        let mut locked_writer = self
            .handle
            .lock()
            .map_err(|_| SharedWriterErr::FailedToLock)?;
        locked_writer.set_cursor_location(coord)
    }
    fn set_foreground_color(
        &mut self,
        color: crate::rendering::colors::TerminalColors,
    ) -> Result<(), super::handle_error::HandleError> {
        let mut locked_writer = self
            .handle
            .lock()
            .map_err(|_| SharedWriterErr::FailedToLock)?;
        locked_writer.set_foreground_color(color)
    }
    fn set_background_color(
        &mut self,
        color: crate::rendering::colors::TerminalColors,
    ) -> Result<(), super::handle_error::HandleError> {
        let mut locked_writer = self
            .handle
            .lock()
            .map_err(|_| SharedWriterErr::FailedToLock)?;
        locked_writer.set_background_color(color)
    }
    fn write_to_location(
        &mut self,
        buf: &[u8],
        coord: crate::shared::usize2d::Coord,
    ) -> Result<usize, HandleError> {
        //TODO: What even
        self.set_cursor_location(coord)?;
        self.write(buf).map_err(|_| HandleError::WriteFailed)
    }
}

#[derive(Debug)]
pub enum SharedWriterErr {
    FailedToLock,
}
impl Error for SharedWriterErr {}
impl From<SharedWriterErr> for io::Error {
    fn from(err: SharedWriterErr) -> io::Error {
        std::io::Error::other(err)
    }
}
impl From<SharedWriterErr> for HandleError {
    fn from(err: SharedWriterErr) -> HandleError {
        match err {
            SharedWriterErr::FailedToLock => HandleError::LockFailed,
        }
    }
}
impl Display for SharedWriterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SharedWriterErr::FailedToLock => write!(f, "failed to lock writer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::handler::memory_handle::MemoryHandle;

    use super::SharedHandle;

    #[test]
    fn hello_world() {
        let buffer = Arc::new(Mutex::new(Box::new(MemoryHandle::default())));
        let writer = SharedHandle::init(buffer.clone());
        let test_str = "Hello world";
        match writer.write(format_args!("{}", test_str)) {
            Ok(_) => (),
            Err(_) => panic!("This should never happen"),
        };
        let result = writer.flush();
        assert!(result.is_ok());

        let unwrapped: Vec<u8> = buffer.lock().unwrap().get_buffer_content();
        let result = String::from_utf8_lossy(&unwrapped);

        assert_eq!(test_str, result);
    }
    // TODO: more testing here
}
