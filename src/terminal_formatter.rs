use std::io::{Stdout, Write};

pub struct Terminal {
    pub buffer: Stdout,
}

pub enum TerminalColors {
    LightGreen = 120,
    Red = 160,
    White = 231,
    Black = 232,
    HotPink = 200,
}

impl Terminal {
    // Initialize an instance of the terminal struct
    //
    // # Example
    // ```
    // let term = terminal_formatter::init();
    // ```
    pub fn init() -> Self {
        Terminal {
            buffer: std::io::stdout(),
        }
    }
    pub fn write(&mut self, val: String) {
        let _ = write!(self.buffer, "{}", val);
    }
    pub fn writeln(&mut self, val: String) {
        let _ = writeln!(self.buffer, "\x1b[2K{}", val);
    }
    pub fn flush(&mut self) {
        let _ = self.buffer.flush();
    }

    // Lock the output
    //
    // # Example
    // ```
    // terminal_formatter.lock();
    // ```
    pub fn lock(&mut self) {
        let _ = self.buffer.lock();
    }

    // Clear the terminal
    //
    // # Example
    // ```
    // terminal_formatter.clear();
    // ```
    pub fn clear(&mut self) {
        self.reset_colors();
        let _ = write!(self.buffer, "\x1B[2J");
        self.reset_colors();
    }
    // Clears the current line
    //
    // # Example
    // ```
    // terminal_formatter.clear_line();
    // ```
    pub fn clear_line(&mut self) {
        self.reset_colors();
        let _ = write!(self.buffer, "\x1B[2K");
        self.reset_colors();
    }
    // Hide the cursor
    //
    // # Example
    // ```
    // terminal_formatter.hide_cursor();
    // ```
    pub fn hide_cursor(&mut self) {
        let _ = write!(self.buffer, "\x1B[?25l");
    }
    // Show the cursor
    //
    // # Example
    // ```
    // terminal_formatter.show_cursor();
    // ```
    pub fn show_cursor(&mut self) {
        let _ = write!(self.buffer, "\x1B[?25h");
    }
    // Prints the ansi characters that sets the terminal background color at the current cursor
    // location
    //
    // # Example
    // ```
    // terminal_formatter.set_background(TerminalColors::HotPink);
    // ```
    pub fn set_background(&mut self, color_code: TerminalColors) {
        let _ = write!(self.buffer, "\x1b[48;5;{}m", color_code as u32);
    }

    // Prints the ansi characters that sets the terminal foreground color at the current cursor
    // location
    //
    // # Example
    // ```
    // terminal_formatter.set_foreground(TerminalColors::HotPink);
    // ```
    pub fn set_foreground(&mut self, color_code: TerminalColors) {
        let _ = write!(self.buffer, "\x1b[38;5;{}m", color_code as u32);
    }

    // Prints the ansi characters that sets the cursor location on the terminal
    //
    // # Example
    // ```
    // terminal_formatter.set_cursor_location(0,0);
    // ```
    pub fn set_cursor_location(&mut self, x: u16, y: u16) {
        let _ = write!(self.buffer, "\x1b[{};{}H", y, x);
    }

    // Reset terminal colors and styles
    // location
    //
    // # Example
    // ```
    // terminal_formatter::reset_colors();
    // ```
    pub fn reset_colors(&mut self) {
        let _ = write!(self.buffer, "\x1B[0m");
    }
}
