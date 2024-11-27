use core::time;
use std::{env, str::FromStr, usize};

use conways_game::PrintMode;

pub mod conways_game;
pub mod conways_law;
pub mod terminal_formatter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let x_len: usize = read_config(&args, "--x-len".to_string(), 10);
    let y_len: usize = read_config(&args, "--y-len".to_string(), 10);
    let seed: u64 = read_config(&args, "--seed".to_string(), 10);
    let print_mode: PrintMode = read_config(&args, "--mode".to_string(), PrintMode::PRETTY);

    let mut gs = conways_game::ConwaysGame::init(x_len, y_len, seed);
    let duration = time::Duration::from_millis(1000);
    gs.run(duration, print_mode);
}

fn read_config<T>(args: &Vec<String>, arg: String, default: T) -> T
where
    T: FromStr + Clone + Copy,
{
    let mut out: T = default.clone();
    for i in 1..args.len() {
        if args[i].as_str() != arg {
            continue;
        }
        if let Some(val) = args.get(i + 1) {
            out = val.parse().unwrap_or_else(|_| {
                eprint!("invalid value for {}. Using default.", arg);
                default
            });
        }
    }

    out
}
