use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use super::{handle::Handle, std_io_handle::StdIOHandle};

pub struct SharedHandle {
    handle: Arc<Mutex<dyn Handle>>,
}

impl SharedHandle {
    pub fn init(writer: Arc<Mutex<dyn Handle>>) -> Self {
        SharedHandle { handle: writer }
    }
    pub fn init_std_out() -> Self {
        SharedHandle {
            handle: Arc::new(Mutex::new(StdIOHandle::new())),
        }
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

#[derive(Debug)]
pub enum SharedWriterErr {
    FailedToLock,
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
        let buffer = Arc::new(Mutex::new(MemoryHandle::new()));
        let writer = SharedHandle::init(buffer.clone());
        let test_str = "Hello world";
        let _ = match writer.write(format_args!("{}", test_str)) {
            Ok(_) => (),
            Err(_) => assert!(false, "This should never happen"),
        };
        let result = writer.flush();
        assert!(result.is_ok());

        let unwrapped: Vec<u8> = buffer.lock().unwrap().get_buffer_content();
        let result = String::from_utf8_lossy(&unwrapped);

        assert_eq!(test_str, result);
    }
}
