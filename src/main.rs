use core::time;
use std::{env, usize};

use conways_game::PrintMode;

pub mod conways_game;
pub mod conways_law;
pub mod terminal_formatter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut x_len: usize = 10;
    let mut y_len: usize = 10;
    let mut seed: u64 = 10;
    let mut print_mode: PrintMode = PrintMode::PRETTY;

    for i in 1..args.len() {
        match args[i].as_str() {
            "--x-len" => {
                if let Some(val) = args.get(i + 1) {
                    x_len = val.parse().unwrap_or_else(|_| {
                        eprint!("invalid value for --x-len. Using default.");
                        x_len
                    });
                }
            }
            "--y-len" => {
                if let Some(val) = args.get(i + 1) {
                    y_len = val.parse().unwrap_or_else(|_| {
                        eprint!("invalid value for --y-len. Using default.");
                        x_len
                    });
                }
            }
            "--seed" => {
                if let Some(val) = args.get(i + 1) {
                    seed = val.parse().unwrap_or_else(|_| {
                        eprint!("invalid value for --seed. Using default.");
                        seed
                    });
                }
            }
            "--mode" => {
                if let Some(val) = args.get(i + 1) {
                    print_mode = val.parse().unwrap_or_else(|_| {
                        eprint!("invalid value for --mode. Using default.");
                        print_mode
                    });
                }
            }
            _ => {}
        }
    }
    let mut gs = conways_game::ConwaysGame::init(x_len, y_len, seed);
    let duration = time::Duration::from_millis(1000);
    gs.run(duration, print_mode);
}
