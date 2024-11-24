pub fn reset_terminal() {
    println!("\x1Bc");
}
pub fn hide_cursor() {
    print!("\x1B[?25l");
}
pub fn show_cursor() {
    print!("\x1B[?25h");
}
pub enum TerminalColors {
    LightGreen = 120,
    Red = 160,
    White = 231,
    Black = 232,
    HotPink = 200,
}

// Prints the ansi characters that sets the terminal background color at the current cursor
// location
//
// # Example
// ```
// terminal_formatter::set_background(TerminalColors::HotPink);
// ```
pub fn set_background(color_code: TerminalColors) {
    print!("\x1b[48;5;{}m", color_code as u32);
}

// Prints the ansi characters that sets the terminal foreground color at the current cursor
// location
//
// # Example
// ```
// terminal_formatter::set_foreground(TerminalColors::HotPink);
// ```
pub fn set_foreground(color_code: TerminalColors) {
    print!("\x1b[38;5;{}m", color_code as u32);
}

// Prints the ansi characters that sets the cursor location on the terminal
//
// # Example
// ```
// terminal_formatter::set_cursor_location(0,0);
// ```
pub fn set_cursor_location(x: u16, y: u16) {
    print!("\x1b[{};{}H", y, x);
}

// Reset terminal colors and styles
// location
//
// # Example
// ```
// terminal_formatter::reset_colors();
// ```
pub fn reset_colors() {
    print!("\x1b[0m");
}
