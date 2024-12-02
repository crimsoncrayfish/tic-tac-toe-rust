use rand::prelude::*;
use rand_chacha;
use std::sync::mpsc::Receiver;
use std::thread::{spawn, JoinHandle};
use std::time::SystemTime;
use std::{i64, time::Duration, usize};
use std::{thread, u64};

use crate::console::input_record::KeyEvent;
use crate::conway::command::Command;
use crate::conway::conways_law;
use crate::terminal_formatter;

use super::print_mode::PrintMode;
use super::settings::ConwaysSettings;

pub struct ConwaysGame {
    current: Vec<Vec<bool>>,
    previous: Vec<Vec<bool>>,
    state: ConwaysState,
    settings: ConwaysSettings,
    out: terminal_formatter::Terminal,
    receiver: Receiver<KeyEvent>,
}

struct ConwaysState {
    fps_last: u64,    //lol
    fps_current: u64, //lol
    latest_input: char,
    latest_command: Command,
    command_count: u64,
    latest_err: String,
    is_paused: bool,
    is_fps_limited: bool,
    rounds: u64,
    print_mode: PrintMode,
}

impl ConwaysGame {
    ///Initialize an instance of ConwaysGame
    ///
    /// # Arguments
    ///
    /// * `x_len` - number of cells in a row
    /// * `y_len` - number of rows in the grid
    /// * `seed` - a seed for the randomness used to do the initialization
    ///
    /// #Returns
    ///
    /// An instance of ConwaysGame
    ///
    /// # Examples
    ///
    /// ```
    /// let game = ConwaysGame::init(1,9001,42);
    /// ```
    pub fn init(
        x_len: usize,
        y_len: usize,
        seed: u64,
        mode: PrintMode,
        duration: Duration,
        receiver: Receiver<KeyEvent>,
    ) -> Self {
        assert!(x_len > 0);
        assert!(y_len > 0);
        let mut _rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        let mut new_state = vec![vec![false; x_len]; y_len];
        let new_prev = new_state.clone();
        for y in 0..y_len {
            for x in 0..x_len {
                new_state[y][x] = _rng.gen();
            }
        }
        ConwaysGame {
            current: new_state.clone(),
            previous: new_prev,
            out: terminal_formatter::Terminal::init(),
            state: ConwaysState {
                print_mode: mode,
                latest_command: Command::NONE,
                command_count: 0,
                latest_input: ' ',
                rounds: 0,
                fps_last: 0,
                fps_current: 0,
                is_paused: false,
                is_fps_limited: false,
                latest_err: "".to_string(),
            },
            receiver,
            settings: ConwaysSettings::init(x_len, y_len, duration),
        }
    }
    pub fn run_async(
        x_len: usize,
        y_len: usize,
        seed: u64,
        print_mode: PrintMode,
        receiver: Receiver<KeyEvent>,
    ) -> JoinHandle<()> {
        let game_closure = move || {
            let mut gs = ConwaysGame::init(
                x_len,
                y_len,
                seed,
                print_mode,
                Duration::from_millis(1000),
                receiver,
            );
            gs.run();
        };
        spawn(game_closure)
    }

    /// Run the game with the specified time between next calls
    ///
    /// # Arguments
    ///
    /// * `duration` - duration between `next` calls
    ///
    /// # Examples
    ///
    /// ```
    /// let game = ConwaysGame::init(1,9001,42);
    /// let dur = Duration::from_millis(1000);
    /// game.run(dur);
    /// ```
    pub fn run(&mut self) {
        {
            self.out.lock();
            self.out.clear();
            self.out.hide_cursor();
        }
        let now = SystemTime::now();
        let mut elapsed_prev_game: Duration = Duration::from_secs(0);
        let mut elapsed_prev_fps_counter: Duration = Duration::from_secs(0);

        loop {
            match self.receiver.try_recv() {
                Ok(cmd) => self.process_key_command(cmd),
                Err(_) => (),
            };
            if self.state.latest_command == Command::QUIT {
                break;
            }
            match now.elapsed() {
                Ok(elapsed) => {
                    let elapsed_millis = elapsed - elapsed_prev_fps_counter;
                    if elapsed_millis > Duration::from_millis(100) {
                        let secs = elapsed_millis.as_secs_f64();
                        self.state.fps_last = (self.state.fps_current as f64 / secs).floor() as u64;
                        self.state.fps_current = 0;
                        elapsed_prev_fps_counter = elapsed.clone();
                    }
                    self.state.fps_current += 1;

                    if !self.state.is_paused
                        && elapsed - elapsed_prev_game > self.settings.round_duration
                    {
                        self.next();
                        self.state.rounds += 1;
                        elapsed_prev_game = elapsed.clone();
                    }
                }
                Err(e) => self.state.latest_err = e.to_string(),
            };
            if self.state.is_fps_limited {
                thread::sleep(Duration::from_millis(16));
            }

            self.print(self.state.print_mode);
            self.out.flush();
            if self.is_stable() {
                break;
            }
        }
        self.out.lock();
        self.out.show_cursor();
        self.out.reset_colors();
    }
    /// Checks if the next and previous frames are the same
    ///
    /// # Examples
    ///
    /// ```
    /// game.is_stable();
    ///
    /// ```
    pub fn is_stable(&self) -> bool {
        self.previous.eq(&self.current)
    }
    /// Processes any incoming commands sent to the game
    ///
    /// # Examples
    ///
    /// ```
    /// self.process_command();
    ///
    /// ```
    fn process_key_command(&mut self, command: KeyEvent) {
        if !command.is_down {
            return;
        }
        self.state.command_count += 1;
        self.state.latest_input = command.command;
        self.state.latest_command = match command.command {
            'q' | 'Q' => Command::QUIT,
            'm' | 'M' => {
                self.state.print_mode = match self.state.print_mode {
                    PrintMode::DEBUG => {
                        self.out.clear();
                        self.out.hide_cursor();
                        PrintMode::PRETTY
                    }
                    PrintMode::PRETTY => {
                        self.out.clear();
                        self.out.hide_cursor();
                        PrintMode::DEBUG
                    }
                };
                Command::TOGGLEMODE
            }
            ' ' => {
                self.state.is_paused = !self.state.is_paused;
                Command::PAUSEPLAY
            }
            'f' | 'F' => {
                self.state.is_fps_limited = !self.state.is_fps_limited;
                Command::TOGGLEFPS
            }
            'w' | 'W' => {
                if self.settings.origin.y > 0 {
                    self.settings.origin.y -= 1;
                    self.clear_cells();
                }
                Command::MOVEUP
            }
            'a' | 'A' => {
                if self.settings.origin.x > 0 {
                    self.settings.origin.x -= 1;
                    self.clear_cells();
                }
                Command::MOVELEFT
            }
            's' | 'S' => {
                self.settings.origin.y += 1;
                self.clear_cells();
                Command::MOVEDOWN
            }
            'd' | 'D' => {
                self.settings.origin.x += 1;
                self.clear_cells();
                Command::MOVERIGHT
            }
            _ => Command::NOMAPPING,
        }
    }

    /// Pretty print the current state
    ///
    /// # Examples
    ///
    /// ```
    /// game.print(PrintMode::Pretty);
    ///
    /// ```
    /// prints the following:
    /// █   █
    ///      
    ///     █
    pub fn print(&mut self, print_mode: PrintMode) {
        self.out.lock();
        for y in 0..self.settings.y_len {
            for x in 0..self.settings.x_len {
                self.print_cell(x as u16, y as u16, self.current[y][x], print_mode);
            }
        }

        if self.state.print_mode == PrintMode::DEBUG {
            self.debug();
        }
    }
    /// Print all debug info
    ///
    /// # Examples
    ///
    /// ```
    /// game.debug();
    ///
    /// ```
    fn debug(&mut self) {
        let x_start = 0;
        let y_start = (self.settings.y_len as u16 * self.settings.cell_view_height)
            + 1
            + self.settings.y_len as u16
            + self.settings.origin.y;
        self.out.set_cursor_location(x_start, y_start);
        self.out
            .set_background(terminal_formatter::TerminalColors::White);
        self.out
            .set_foreground(terminal_formatter::TerminalColors::Red);
        self.out.writeln(format!("Round {}. ", self.state.rounds));
        self.out.writeln(format!(
            "Latest Command: cmd - '{}', input - '{}'",
            self.state.latest_command, self.state.latest_input
        ));

        self.out
            .writeln(format!("Cmd count: {}", self.state.command_count));
        self.out.writeln(format!("Mode: {}", self.state.print_mode));
        self.out
            .writeln(format!("Is Paused: {}", self.state.is_paused));
        self.out
            .writeln(format!("FPS Count: {}", self.state.fps_last));

        if self.state.latest_err != "" {
            self.out
                .set_background(terminal_formatter::TerminalColors::Red);
            self.out
                .set_foreground(terminal_formatter::TerminalColors::White);
            self.out
                .writeln(format!("Error: {}", self.state.latest_err));
        }
    }

    /// print the cell as either the full debug info or just a block
    ///
    /// # Examples
    ///
    /// ```
    /// game.print_cell(0, 0, true, PrintMode::Pretty);
    ///
    /// ```
    /// prints the following:
    /// ███
    /// ███
    ///
    /// ```
    /// game.print_cell(9, 3, true, PrintMode::Debug);
    ///
    /// ```
    /// prints the following:
    /// 012 true
    /// 345 9:3
    ///
    fn print_cell(&mut self, x: u16, y: u16, is_alive: bool, print_mode: PrintMode) {
        let debug_width: u16 = 6;
        let mut x_start = x * self.settings.cell_view_width + 1 + x + self.settings.origin.x;
        let y_start = (y * self.settings.cell_view_height) + 1 + y + self.settings.origin.y;
        match print_mode {
            PrintMode::PRETTY => {
                for y_offset in 0..self.settings.cell_view_height {
                    for x_offset in 0..self.settings.cell_view_width {
                        self.out
                            .set_cursor_location(x_start + x_offset, y_start + y_offset);
                        if is_alive {
                            self.out
                                .set_background(terminal_formatter::TerminalColors::LightGreen);
                            self.out
                                .set_foreground(terminal_formatter::TerminalColors::Black);
                        } else {
                            self.out
                                .set_background(terminal_formatter::TerminalColors::Red);
                            self.out
                                .set_foreground(terminal_formatter::TerminalColors::White);
                        }
                        self.out.write(" ".to_string());
                    }
                }
            }
            PrintMode::DEBUG => {
                x_start += x * debug_width;
                for y_offset in 0..self.settings.cell_view_height {
                    for x_offset in 0..self.settings.cell_view_width + debug_width {
                        // TODO: this can be improved
                        // i dont need to set the background here every time
                        self.out
                            .set_cursor_location(x_start + x_offset, y_start + y_offset);
                        if x_offset == 0 {
                            if is_alive {
                                self.out
                                    .set_background(terminal_formatter::TerminalColors::LightGreen);
                                self.out
                                    .set_foreground(terminal_formatter::TerminalColors::Black);
                            } else {
                                self.out
                                    .set_background(terminal_formatter::TerminalColors::Red);
                                self.out
                                    .set_foreground(terminal_formatter::TerminalColors::White);
                            }
                        }

                        if x_offset < self.settings.cell_view_width {
                            self.out.write(format!(
                                "{}",
                                y_offset * self.settings.cell_view_height + x_offset + y_offset
                            ));
                        } else {
                            self.out.write(" ".to_string());
                        }
                    }
                }

                self.out
                    .set_cursor_location(x_start + self.settings.cell_view_width, y_start);

                if is_alive {
                    self.out
                        .set_background(terminal_formatter::TerminalColors::LightGreen);
                    self.out
                        .set_foreground(terminal_formatter::TerminalColors::Black);
                    self.out.write(" true ".to_string());
                } else {
                    self.out
                        .set_background(terminal_formatter::TerminalColors::Red);
                    self.out
                        .set_foreground(terminal_formatter::TerminalColors::White);
                    self.out.write(" false".to_string());
                }
                self.out
                    .set_cursor_location(x_start + self.settings.cell_view_width, y_start + 1);

                if is_alive {
                    self.out
                        .set_background(terminal_formatter::TerminalColors::LightGreen);
                    self.out
                        .set_foreground(terminal_formatter::TerminalColors::Black);
                } else {
                    self.out
                        .set_background(terminal_formatter::TerminalColors::Red);
                    self.out
                        .set_foreground(terminal_formatter::TerminalColors::White);
                }
                self.out.write(format!(" {}:{}", x, y));
            }
        }
        self.out.flush();
    }
    /// Calculate and apply the next frame, while the calculations are running the current and the
    /// previous are the same
    ///
    /// # Examples
    ///
    /// ```
    /// game.next();
    /// ```
    pub fn next(&mut self) {
        self.previous = self.current.clone();
        let mut new_state = vec![vec![false; self.settings.x_len]; self.settings.y_len];
        for y in 0..self.settings.y_len {
            for x in 0..self.settings.x_len {
                let live_siblings = self.count_siblings(x, y);
                new_state[y][x] = conways_law::conways_law(self.current[y][x], live_siblings);
            }
            assert_eq!(
                self.settings.x_len,
                new_state[y].len(),
                "the length of the row should not change"
            );
        }
        assert_eq!(
            self.settings.y_len,
            new_state.len(),
            "the number of rows should not change"
        );
        self.current = new_state;
    }
    /// Count the number of living siblings at a location on the previous state
    ///
    /// # Arguments
    ///
    /// * `x` - x location
    /// * `y` - y location
    ///
    /// # Examples
    ///
    /// ```
    /// let siblings = game.count_siblings(1,4);
    /// ```
    pub fn count_siblings(&mut self, x_location: usize, y_location: usize) -> u8 {
        let mut sibling_count = 0;
        for x_delta in -1i64..=1 {
            for y_delta in -1i64..=1 {
                if x_delta == 0 && y_delta == 0 {
                    continue;
                }
                let mut x_sibling = (x_location as i64) + x_delta;
                if x_sibling < 0 {
                    x_sibling = (self.settings.x_len as i64) - 1;
                } else if x_sibling >= self.settings.x_len as i64 {
                    x_sibling = 0;
                }
                let mut y_sibling = (y_location as i64) + y_delta;
                if y_sibling < 0 {
                    y_sibling = (self.settings.y_len as i64) - 1;
                } else if y_sibling >= self.settings.y_len as i64 {
                    y_sibling = 0;
                }
                assert!(x_sibling >= 0);
                assert!(y_sibling >= 0);
                assert!(x_sibling < self.settings.x_len as i64);
                assert!(y_sibling < self.settings.y_len as i64);
                if self.current[y_sibling as usize][x_sibling as usize] {
                    sibling_count += 1;
                }
            }
        }
        sibling_count
    }
    // Clear the screen manually
    // TODO: this is a temporary fix so i dont have to call clear so often
    //
    // ### Usage
    //
    // ```
    // self.clear_cells();
    // ```
    fn clear_cells(&mut self) {
        let total_height = (self.settings.y_len as u16 * self.settings.cell_view_height)
            + 1
            + self.settings.y_len as u16
            + self.settings.origin.y;
        for y_loc in 0..total_height {
            self.out.set_cursor_location(0, y_loc);
            self.out.clear_line();
        }
        if self.state.print_mode == PrintMode::DEBUG {
            //TODO: this hardcoded 6 is painfull to see
            for y_loc in total_height + 1..total_height + 7 {
                self.out.set_cursor_location(0, y_loc);
                self.out.clear_line();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, time::Duration};

    use super::ConwaysGame;

    #[test]
    fn init_works() {
        let (_sen, rec) = mpsc::channel();
        let game = ConwaysGame::init(
            5,
            5,
            55,
            super::PrintMode::DEBUG,
            Duration::from_secs(1),
            rec,
        );

        let current_state = game.current.clone();

        let scenarios = [
            (0, 0, true),
            (1, 0, true),
            (2, 0, true),
            (3, 0, false),
            (4, 0, true),
            (0, 1, true),
            (1, 1, false),
            (2, 1, false),
            (3, 1, false),
            (4, 1, true),
            (0, 2, false),
            (1, 2, true),
            (2, 2, false),
            (3, 2, true),
            (4, 2, false),
            (0, 3, true),
            (1, 3, true),
            (2, 3, false),
            (3, 3, true),
            (4, 3, true),
            (0, 4, true),
            (1, 4, false),
            (2, 4, false),
            (3, 4, false),
            (4, 4, true),
        ];

        for (x, y, expected) in scenarios {
            assert_eq!(
                current_state[y][x], expected,
                "x,y: {},{}, state: {}",
                x, y, expected
            );
        }
    }
    #[test]
    fn next() {
        let (_sen, rec) = mpsc::channel();
        let mut game = ConwaysGame::init(
            5,
            5,
            55,
            super::PrintMode::DEBUG,
            Duration::from_secs(1),
            rec,
        );
        game.next();

        let scenarios = [
            (0, 0, false),
            (1, 0, false),
            (2, 0, false),
            (3, 0, false),
            (4, 0, false),
            (0, 1, false),
            (1, 1, false),
            (2, 1, false),
            (3, 1, false),
            (4, 1, false),
            (0, 2, false),
            (1, 2, true),
            (2, 2, false),
            (3, 2, true),
            (4, 2, false),
            (0, 3, false),
            (1, 3, true),
            (2, 3, false),
            (3, 3, true),
            (4, 3, false),
            (0, 4, false),
            (1, 4, false),
            (2, 4, false),
            (3, 4, false),
            (4, 4, false),
        ];
        let current_state = game.current.clone();

        for (x, y, expected) in scenarios {
            assert_eq!(
                current_state[y][x], expected,
                "x,y: {},{}, state expected: {}, actual: {}",
                x, y, expected, current_state[y][x]
            );
        }
    }
    #[test]
    fn siblings_count() {
        let (_sen, rec) = mpsc::channel();
        let mut game = ConwaysGame::init(
            5,
            5,
            55,
            super::PrintMode::DEBUG,
            Duration::from_secs(1),
            rec,
        );

        let scenarios = [
            (0, 0, 6),
            (0, 1, 5),
            (0, 2, 6),
            (0, 3, 5),
            (0, 4, 7),
            (1, 0, 4),
            (1, 1, 5),
            (1, 2, 3),
            (1, 3, 3),
            (1, 4, 6),
            (2, 0, 1),
            (2, 1, 4),
            (2, 2, 4),
            (2, 3, 4),
            (2, 4, 4),
            (3, 0, 4),
            (3, 1, 4),
            (3, 2, 3),
            (3, 3, 3),
            (3, 4, 5),
            (4, 0, 5),
            (4, 1, 4),
            (4, 2, 6),
            (4, 3, 5),
            (4, 4, 6),
        ];

        for (x, y, expected) in scenarios {
            assert_eq!(
                game.count_siblings(x, y),
                expected,
                "x,y: {},{}, state expected: {}, actual: {}",
                x,
                y,
                expected,
                game.count_siblings(x, y)
            );
        }
    }
}
