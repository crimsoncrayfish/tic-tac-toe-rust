use super::shared_writer::SharedWriter;

pub struct Terminal {
    pub writer: SharedWriter,
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
    pub fn init(w: SharedWriter) -> Self {
        Terminal { writer: w }
    }
    pub fn write(&mut self, val: String) {
        let _ = match self.writer.write(format_args!("{}", val)) {
            Ok(_) => (),
            Err(_) => assert!(false, "TODO: Handle write failures"),
        };
    }
    pub fn writeln(&mut self, val: String) {
        let _ = match self.writer.write(format_args!("\x1b[2K{}", val)) {
            Ok(_) => (),
            Err(_) => assert!(false, "TODO: Handle write failures"),
        };
    }
    pub fn flush(&mut self) {
        let _ = match self.writer.flush() {
            Ok(_) => (),
            Err(_) => assert!(false, "TODO: Handle flush failures"),
        };
    }

    // Clear the terminal
    //
    // # Example
    // ```
    // terminal_formatter.clear();
    // ```
    pub fn clear(&mut self) {
        self.reset_colors();
        self.write("\x1B[2J".to_string());
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
        self.write("\x1B[2K".to_string());
        self.reset_colors();
    }
    // Hide the cursor
    //
    // # Example
    // ```
    // terminal_formatter.hide_cursor();
    // ```
    pub fn hide_cursor(&mut self) {
        self.write("\x1B[?25l".to_string());
    }
    // Show the cursor
    //
    // # Example
    // ```
    // terminal_formatter.show_cursor();
    // ```
    pub fn show_cursor(&mut self) {
        self.write("\x1B[?25h".to_string());
    }
    // Prints the ansi characters that sets the terminal background color at the current cursor
    // location
    //
    // # Example
    // ```
    // terminal_formatter.set_background(TerminalColors::HotPink);
    // ```
    pub fn set_background(&mut self, color_code: TerminalColors) {
        let _ = match self
            .writer
            .write(format_args!("\x1b[48;5;{}m", color_code as u32))
        {
            Ok(_) => (),
            Err(_) => assert!(false, "TODO: Handle write failures"),
        };
    }

    // Prints the ansi characters that sets the terminal foreground color at the current cursor
    // location
    //
    // # Example
    // ```
    // terminal_formatter.set_foreground(TerminalColors::HotPink);
    // ```
    pub fn set_foreground(&mut self, color_code: TerminalColors) {
        let _ = match self
            .writer
            .write(format_args!("\x1b[38;5;{}m", color_code as u32))
        {
            Ok(_) => (),
            Err(_) => assert!(false, "TODO: Handle write failures"),
        };
    }

    // Prints the ansi characters that sets the cursor location on the terminal
    //
    // # Example
    // ```
    // terminal_formatter.set_cursor_location(0,0);
    // ```
    pub fn set_cursor_location(&mut self, x: u16, y: u16) {
        let _ = match self.writer.write(format_args!("\x1b[{};{}H", y, x)) {
            Ok(_) => (),
            Err(_) => assert!(false, "TODO: Handle write failures"),
        };
    }

    // Reset terminal colors and styles
    // location
    //
    // # Example
    // ```
    // terminal_formatter::reset_colors();
    // ```
    pub fn reset_colors(&mut self) {
        self.write("\x1B[0m".to_string());
    }
}
