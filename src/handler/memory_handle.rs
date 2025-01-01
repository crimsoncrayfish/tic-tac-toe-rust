use std::{
    fmt::Debug,
    io::{self, Write},
    usize,
};

use crate::{
    rendering::colors::TerminalColors,
    shared::usize2d::{Coord, Usize2d},
    utils::vec_t_writer::{write_t_to_vec, write_vec_to_vec},
};

use super::{handle::Handle, handle_error::HandleError};

/// The behavior of `MemoryHandle` shoudld be as similar to the `StdOut` behaviour as possible.
/// It is used for unit/simulation testing
pub struct MemoryHandle {
    pub buffer: Vec<Vec<u8>>,
    buffer_temp: Vec<Vec<u8>>,
    foreground_color_buffer: Vec<Vec<TerminalColors>>,
    foreground_color_buffer_temp: Vec<Vec<TerminalColors>>,
    background_color_buffer: Vec<Vec<TerminalColors>>,
    background_color_buffer_temp: Vec<Vec<TerminalColors>>,
    current_cursor_location: Usize2d,
    current_background_color: TerminalColors,
    current_foreground_color: TerminalColors,
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
            current_cursor_location: Usize2d::default(),
            current_background_color: TerminalColors::default(),
            current_foreground_color: TerminalColors::default(),
        }
    }
}
impl MemoryHandle {
    pub fn get_buffer_content(&self) -> Vec<u8> {
        if self.buffer.len() == 0 {
            return Vec::new();
        }
        assert!(self.buffer.len() > 0);
        let mut result: Vec<u8> = Vec::with_capacity(self.buffer_temp.len() * 2 - 1);

        for index in 0..self.buffer_temp.len() {
            result.extend(self.buffer_temp[index].clone());
            if index < self.buffer_temp.len() - 1 {
                result.push(b'\n');
            }
        }

        result
    }
    pub fn get_background_colors(&self) -> Vec<Vec<TerminalColors>> {
        self.background_color_buffer.clone()
    }
    pub fn get_foreground_colors(&self) -> Vec<Vec<TerminalColors>> {
        self.foreground_color_buffer.clone()
    }
    pub fn need_to_flush(self) -> bool {
        self.buffer_temp != self.buffer
    }
}

impl Debug for MemoryHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[MemoryHandle {{ cursor_location: {}, background_color: {}, foreground_color: {}, buffer_size: {} }}]",
            self.current_cursor_location,
            self.current_background_color,
            self.current_foreground_color,
            self.buffer.iter().map(|s| s.len()).sum::<usize>()
        )
    }
}
impl Write for MemoryHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let vec_to_push = buf.to_vec();
        let len_to_push = vec_to_push.len();

        let required_len = self.current_cursor_location.y + 1;
        if self.buffer_temp.len() < required_len {
            self.buffer_temp.resize_with(required_len, Vec::new);
            self.background_color_buffer_temp
                .resize_with(required_len, Vec::new);
            self.foreground_color_buffer_temp
                .resize_with(required_len, Vec::new);
        }
        self.buffer_temp[self.current_cursor_location.y] = write_vec_to_vec(
            self.buffer_temp[self.current_cursor_location.y].clone(),
            vec_to_push,
            self.current_cursor_location.x,
            b' ',
        );

        self.background_color_buffer_temp[self.current_cursor_location.y] = write_t_to_vec(
            self.background_color_buffer_temp[self.current_cursor_location.y].clone(),
            self.current_background_color,
            len_to_push,
            self.current_cursor_location.x,
            TerminalColors::default(),
        );

        self.foreground_color_buffer_temp[self.current_cursor_location.y] = write_t_to_vec(
            self.foreground_color_buffer_temp[self.current_cursor_location.y].clone(),
            self.current_foreground_color,
            len_to_push,
            self.current_cursor_location.x,
            TerminalColors::default(),
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
    fn set_cursor_location(&mut self, coordinate: Usize2d) -> Result<(), HandleError> {
        self.current_cursor_location = coordinate;
        Ok(())
    }
    fn set_foreground_color(&mut self, color: TerminalColors) -> Result<(), HandleError> {
        self.current_foreground_color = color;
        Ok(())
    }
    fn set_background_color(&mut self, color: TerminalColors) -> Result<(), HandleError> {
        self.current_background_color = color;
        Ok(())
    }
    fn write_to_location(&mut self, buf: &[u8], coord: Coord) -> Result<usize, HandleError> {
        let _ = self.set_cursor_location(coord)?;
        self.write(buf).map_err(|_| HandleError::WriteFailed)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::rendering::colors::TerminalColors as TC;
    use crate::{
        handler::{handle::Handle, memory_handle::MemoryHandle},
        shared::usize2d::Usize2d,
        vec_vec_enum_to_string,
    };

    #[test]
    fn hello_world() {
        let mut handle = MemoryHandle::new();
        let test_str: &[u8] = b"Hello world";
        let result = handle.write(test_str);
        assert!(result.is_ok(), "Write should not fail");

        let result = handle.flush();
        assert!(result.is_ok(), "Flush should not fail");

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
        let result = handle.write(test_str);
        assert!(result.is_ok(), "Write should not fail");

        let result = handle.flush();
        assert!(result.is_ok(), "Flush should not fail");

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
        let result = handle.write(test_str);
        assert!(result.is_ok(), "Write should not fail");

        let result = handle.flush();
        assert!(result.is_ok(), "Flush should not fail");

        let buffer_content: Vec<u8> = handle.get_buffer_content();
        let result = String::from_utf8_lossy(&buffer_content);

        let expected = "\n\n     Hello world".to_string();

        assert_eq!(
            expected, result,
            "The initial string written to the MemoryHandle should match to output. Got: \'{}\', Expected: \'{}\'",
            result, expected
        );
    }
    #[test]
    fn set_colors() {
        let mut handle = MemoryHandle::new();

        let result = handle.set_foreground_color(TC::Red);
        assert!(result.is_ok());
        assert_eq!(
            handle.current_foreground_color,
            TC::Red,
            "Expected foreground color: {}, Got: {}",
            handle.current_foreground_color,
            TC::Red
        );

        let result = handle.set_background_color(TC::Black);
        assert!(result.is_ok());
        assert_eq!(
            handle.current_background_color,
            TC::Black,
            "Expected background color: {}, Got: {}",
            handle.current_background_color,
            TC::Black
        );

        let result = handle.set_cursor_location(Usize2d::new(5, 2));
        assert!(result.is_ok());

        let test_str: &[u8] = b"Hello world";
        let result = handle.write(test_str);
        assert!(result.is_ok(), "Write should not fail");

        let result = handle.flush();
        assert!(result.is_ok(), "Flush should not fail");

        let buffer_content: Vec<u8> = handle.get_buffer_content();
        let result = String::from_utf8_lossy(&buffer_content);

        let expected = "\n\n     Hello world".to_string();

        assert_eq!(
            expected, result,
            "The initial string written to the MemoryHandle should match to output. Got: \'{}\', Expected: \'{}\'",
            result, expected
        );
        let background_colors = handle.get_background_colors();
        let expected: Vec<Vec<TC>> = vec![
            vec![], //
            vec![], //
            vec![
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Black,
                TC::Black,
                TC::Black,
                TC::Black,
                TC::Black,
            ], //
        ];

        assert_eq!(
            expected,
            background_colors,
            "The background colors should match the expected colors. Got: \'{}\', Expected: \'{}\'",
            vec_vec_enum_to_string!(background_colors),
            vec_vec_enum_to_string!(expected)
        );
        let foreground_colors = handle.get_foreground_colors();
        let expected: Vec<Vec<TC>> = vec![
            vec![], //
            vec![], //
            vec![
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Default,
                TC::Red,
                TC::Red,
                TC::Red,
                TC::Red,
                TC::Red,
            ], //
        ];

        assert_eq!(
            expected,
            foreground_colors,
            "The foreground colors should match the expected colors. Got: \'{}\', Expected: \'{}\'",
            vec_vec_enum_to_string!(foreground_colors),
            vec_vec_enum_to_string!(expected)
        );
    }
}
