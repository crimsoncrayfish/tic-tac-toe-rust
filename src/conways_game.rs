use rand::prelude::*;
use rand_chacha;
use std::str::FromStr;
use std::thread::sleep;
use std::{i64, time::Duration, usize};

use crate::conways_law::conways_law;
use crate::terminal_formatter;

pub enum GameState {
    INITIALIZED,
    RUNNING,
    PAUSED,
    DONE,
}
#[derive(PartialEq, Clone, Copy)]
pub enum PrintMode {
    PRETTY,
    DEBUG,
}
impl FromStr for PrintMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pretty" | "PRETTY" => Ok(PrintMode::PRETTY),
            "debug" | "DEBUG" => Ok(PrintMode::DEBUG),
            _ => Err(format!("`{}` is not a valid mode", s)),
        }
    }
}

pub struct ConwaysGame {
    current: Vec<Vec<bool>>,
    previous: Vec<Vec<bool>>,
    state: GameState,
    rounds: u64,
    settings: ConwaysSettings,
    out: terminal_formatter::Terminal,
}
pub struct ConwaysSettings {
    x_len: usize,
    y_len: usize,
    cell_view_width: u16,
    cell_view_height: u16,
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
    pub fn init(x_len: usize, y_len: usize, seed: u64) -> Self {
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
            state: GameState::INITIALIZED,
            rounds: 0,
            settings: ConwaysSettings {
                x_len,
                y_len,
                cell_view_width: 3,
                cell_view_height: 2,
            },
        }
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
    pub fn run(&mut self, duration: Duration, print_mode: PrintMode) {
        {
            self.out.lock();
            self.out.clear();
            self.out.hide_cursor();
        }
        while !matches!(self.state, GameState::DONE) {
            sleep(duration);
            self.next();
            self.print(print_mode);
            self.out.flush();
            self.rounds += 1;
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
    pub fn is_stable(&mut self) -> bool {
        self.previous.eq(&self.current)
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
    /// X O X
    /// O O O
    /// O O X
    pub fn print(&mut self, print_mode: PrintMode) {
        self.out.lock();
        for y in 0..self.settings.y_len {
            for x in 0..self.settings.x_len {
                self.print_cell(x as u16, y as u16, self.current[y][x], print_mode);
            }
        }
        if print_mode == PrintMode::DEBUG {
            let x_start = 0;
            let y_start = (self.settings.y_len as u16 * self.settings.cell_view_height)
                + 1
                + self.settings.y_len as u16;
            self.out.set_cursor_location(x_start, y_start);
            self.out
                .set_background(terminal_formatter::TerminalColors::White);
            self.out
                .set_foreground(terminal_formatter::TerminalColors::Red);
            self.out.write(format!("Round {}", self.rounds));
        }
    }
    fn print_cell(&mut self, x: u16, y: u16, is_alive: bool, print_mode: PrintMode) {
        let debug_width: u16 = 6;
        match print_mode {
            PrintMode::PRETTY => {
                let x_start = x * self.settings.cell_view_width + 1 + x;
                let y_start = (y * self.settings.cell_view_height) + 1 + y;
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
                let x_start = x * self.settings.cell_view_width + 1 + x + x * debug_width;
                let y_start = (y * self.settings.cell_view_height) + 1 + y;
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
                        self.out.write(format!(
                            "{}",
                            y_offset * self.settings.cell_view_height + x_offset + y_offset
                        ));
                    }
                }
                for y_offset in 0..self.settings.cell_view_height {
                    for x_offset in 0..debug_width {
                        self.out.set_cursor_location(
                            x_start + self.settings.cell_view_width + x_offset,
                            y_start + y_offset,
                        );
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
                new_state[y][x] = conways_law(self.current[y][x], live_siblings);
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
}

#[cfg(test)]
mod tests {
    use super::ConwaysGame;

    #[test]
    fn init_works() {
        let game = ConwaysGame::init(5, 5, 55);

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
        let mut game = ConwaysGame::init(5, 5, 55);
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
        let mut game = ConwaysGame::init(5, 5, 55);

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
