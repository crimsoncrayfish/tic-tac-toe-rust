use std::{
    fmt::Display,
    io::Write,
    sync::{Arc, Mutex},
};

pub struct SharedWriter {
    writer: Arc<Mutex<dyn Write + Send>>,
}

impl SharedWriter {
    pub fn init(writer: Arc<Mutex<dyn Write + Send>>) -> Self {
        SharedWriter { writer }
    }
    pub fn init_std_out() -> Self {
        SharedWriter {
            writer: Arc::new(Mutex::new(std::io::stdout())),
        }
    }
    pub fn write(&self, args: std::fmt::Arguments) -> Result<(), SharedWriterErr> {
        let mut locked_writer = match self.writer.lock() {
            Ok(result) => result,
            Err(_) => return Err(SharedWriterErr::FailedToLock),
        };
        let _ = locked_writer.write_fmt(args);
        Ok(())
    }
    pub fn flush(&self) -> Result<(), SharedWriterErr> {
        let mut locked_writer = match self.writer.lock() {
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

    use super::SharedWriter;

    #[test]
    fn hello_world() {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let writer = SharedWriter::init(buffer.clone());
        let test_str = "Hello world";
        writer.write(format_args!("{}", test_str));

        let unwrapped = buffer.lock().unwrap();
        let result = String::from_utf8_lossy(&unwrapped);

        assert_eq!(test_str, result);
    }
}
