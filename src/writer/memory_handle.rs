use std::{
    fmt::Debug,
    io::{self, Write},
    usize,
};

use crate::{
    rendering::colors::TerminalColors, shared::usize2d::Usize2d,
    utils::vec_u8_writer::write_to_location,
};

use super::handle::Handle;

pub struct MemoryHandle {
    pub buffer: Vec<Vec<u8>>,
    buffer_temp: Vec<Vec<u8>>,
    foreground_color_buffer: Vec<TerminalColors>,
    foreground_color_buffer_temp: Vec<TerminalColors>,
    background_color_buffer: Vec<TerminalColors>,
    background_color_buffer_temp: Vec<TerminalColors>,
    cursor_location: Usize2d,
}

impl MemoryHandle {
    pub fn new() -> Self {
        MemoryHandle {
            buffer: Vec::new(),
            buffer_temp: Vec::new(),
            foreground_color_buffer: Vec::new(),
            foreground_color_buffer_temp: Vec::new(),
            background_color_buffer: Vec::new(),
            background_color_buffer_temp: Vec::new(),
            cursor_location: Usize2d::default(),
        }
    }
}
impl MemoryHandle {
    pub fn get_buffer_content(&mut self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new(); // TODO: could define with capacity here

        for index in 0..self.buffer_temp.len() {
            result.append(&mut self.buffer_temp[index].clone());
            if index < self.buffer_temp.len() - 1 {
                result.push(b'\n');
            }
        }

        result
    }
    pub fn need_to_flush(self) -> bool {
        self.buffer_temp.len() > 0
    }
}

impl Debug for MemoryHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[MemoryHandle {{ cursor_location: {}, buffer_size: {} }}]",
            self.cursor_location,
            self.buffer.iter().map(|s| s.len()).sum::<usize>()
        )
    }
}
impl Write for MemoryHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let vec_to_push = buf.to_vec();
        while self.buffer_temp.len() <= self.cursor_location.y {
            self.buffer_temp.push(Vec::new());
        }
        self.buffer_temp[self.cursor_location.y] = write_to_location(
            self.buffer_temp[self.cursor_location.y].clone(),
            vec_to_push,
            self.cursor_location.x,
        );

        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.buffer = self.buffer_temp.clone();
        self.foreground_color_buffer = self.foreground_color_buffer_temp.clone();
        self.background_color_buffer = self.background_color_buffer_temp.clone();
        Ok(())
    }
    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()> {
        let mut formatted_string: Vec<u8> = Vec::new();

        match formatted_string.write_fmt(fmt) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        self.buffer_temp.push(formatted_string);
        Ok(())
    }
}

impl Handle for MemoryHandle {
    fn set_cursor_location(
        &mut self,
        coordinate: Usize2d,
    ) -> Result<(), super::handle_error::HandleError> {
        self.cursor_location = coordinate;
        Ok(())
    }
    fn set_foreground_color(
        &mut self,
        color: TerminalColors,
    ) -> Result<(), super::handle_error::HandleError> {
        self.foreground_color_buffer.push(color);
        Ok(())
    }
    fn set_background_color(
        &mut self,
        color: TerminalColors,
    ) -> Result<(), super::handle_error::HandleError> {
        self.background_color_buffer.push(color);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::{
        shared::usize2d::Usize2d,
        writer::{handle::Handle, memory_handle::MemoryHandle},
    };

    #[test]
    fn hello_world() {
        let mut handle = MemoryHandle::new();
        let test_str: &[u8] = b"Hello world";
        let _ = match handle.write(test_str) {
            Ok(_) => (),
            Err(_) => assert!(false, "This should never happen"),
        };
        let result = handle.flush();
        assert!(result.is_ok());

        let buffer_content: Vec<u8> = handle.get_buffer_content();
        let result = String::from_utf8_lossy(&buffer_content);

        let expected = String::from_utf8_lossy(test_str);

        assert_eq!(
            expected, result,
            "The initial string written to the MemoryHandle should match to output"
        );
    }

    #[test]
    fn hello_rust() {
        let mut handle = MemoryHandle::new();
        let test_str: &[u8] = b"Hello world";
        let write_result = handle.write(test_str);
        assert!(write_result.is_ok());

        let set_cursor_result = handle.set_cursor_location(Usize2d::new(6, 0));
        assert!(set_cursor_result.is_ok());

        let test_str: &[u8] = b"rust ";
        let write_result = handle.write(test_str);
        assert!(write_result.is_ok());
        let result = handle.flush();
        assert!(result.is_ok());

        let buffer_content: Vec<u8> = handle.get_buffer_content();
        let result = String::from_utf8_lossy(&buffer_content);

        let expected = String::from_utf8_lossy(b"Hello rust ");

        assert_eq!(
            expected, result,
            "The initial string written to the MemoryHandle should match to output"
        );
    }

    #[test]
    fn set_location() {
        let mut handle = MemoryHandle::new();

        let result = handle.set_cursor_location(Usize2d::new(5, 2));
        assert!(result.is_ok());

        let test_str: &[u8] = b"Hello world";
        let _ = match handle.write(test_str) {
            Ok(_) => (),
            Err(_) => assert!(false, "This should never happen"),
        };
        let result = handle.flush();
        assert!(result.is_ok());

        let buffer_content: Vec<u8> = handle.get_buffer_content();
        let result = String::from_utf8_lossy(&buffer_content);

        let expected = "\n\n     Hello world".to_string();

        assert_eq!(
            expected, result,
            "The initial string written to the MemoryHandle should match to output. Got: \'{}\', Expected: \'{}\'",
            result, expected
        );
    }
}
