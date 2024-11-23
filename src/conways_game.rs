use std::{i64, usize};

use crate::conways_law;
use rand::prelude::*;
use rand_chacha;

#[allow(dead_code)]
pub struct ConwaysGame {
    _x_len: usize,
    _y_len: usize,
    current: Vec<Vec<bool>>,
    previous: Vec<Vec<bool>>,
}
#[allow(dead_code)]
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
        for y in 0..y_len {
            for x in 0..x_len {
                new_state[y][x] = _rng.gen();
            }
        }
        ConwaysGame {
            current: new_state.clone(),
            previous: new_state.clone(),
            _x_len: x_len,
            _y_len: y_len,
        }
    }

    /// Print the current state in the console
    ///
    /// # Examples
    ///
    /// ```
    /// game.debug_print();
    ///
    /// ```
    /// prints the following:
    /// X O X
    /// O O O
    /// O O X
    pub fn debug_print(&mut self) {
        for y in 0..self._y_len {
            for x in 0..self._x_len {
                let cell = self.current[y][x];
                if cell {
                    print!(
                        "\x1b[38;5;232;48;5;120m X |x:{},y:{},sib:{}| ",
                        x,
                        y,
                        self.count_siblings(x, y)
                    );
                } else {
                    print!(
                        "\x1b[38;5;231;48;5;160m O |x:{},y:{},sib:{}| ",
                        x,
                        y,
                        self.count_siblings(x, y)
                    );
                }
            }
            print!("\n");
        }
    }
    /// Calculate and apply the next frame
    ///
    /// # Examples
    ///
    /// ```
    /// game.next();
    /// ```
    pub fn next(&mut self) {
        self.previous = self.current.clone();
        let mut new_state = vec![vec![false; self._x_len]; self._y_len];
        for y in 0..self._y_len {
            for x in 0..self._x_len {
                let live_siblings = self.count_siblings(x, y);
                new_state[y][x] = conways_law::conways_law(self.current[y][x], live_siblings);
            }
            assert_eq!(
                self._x_len,
                new_state[y].len(),
                "the length of the row should not change"
            );
        }
        assert_eq!(
            self._y_len,
            new_state.len(),
            "the number of rows should not change"
        );
        self.current = new_state;
    }
    /// Count the number of living siblings at a location
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
                    x_sibling = (self._x_len as i64) - 1;
                } else if x_sibling >= self._x_len as i64 {
                    x_sibling = 0;
                }
                let mut y_sibling = (y_location as i64) + y_delta;
                if y_sibling < 0 {
                    y_sibling = (self._y_len as i64) - 1;
                } else if y_sibling >= self._y_len as i64 {
                    y_sibling = 0;
                }
                assert!(x_sibling >= 0);
                assert!(y_sibling >= 0);
                assert!(x_sibling < self._x_len as i64);
                assert!(y_sibling < self._y_len as i64);
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
