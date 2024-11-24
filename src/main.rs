use core::time;
use std::{env, usize};

pub mod conways_game;
pub mod conways_law;
pub mod terminal_formatter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let x_len: usize = args[1].parse().expect("Not a valid number");
    let y_len: usize = args[2].parse().expect("Not a valid number");
    let seed: u64 = args[3].parse().expect("Not a valid number");
    let mut gs = conways_game::ConwaysGame::init(x_len, y_len, seed);
    let duration = time::Duration::from_millis(1000);
    gs.run(duration);
}
