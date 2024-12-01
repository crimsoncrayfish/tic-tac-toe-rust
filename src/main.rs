use std::{
    env,
    str::FromStr,
    sync::mpsc::{self},
};

use console::errors::ConsoleControlErr;
use console::notify_inputs;
use conway::{conways_game, print_mode::PrintMode};

pub mod console {
    pub mod console_control;
    pub mod errors;
    pub mod mode;
    pub mod notify_inputs;
}
pub mod conway {
    pub mod command;
    pub mod conways_game;
    pub mod conways_law;
    pub mod print_mode;
    pub mod settings;
}

pub mod coordinate;
pub mod terminal_formatter;

fn main() -> Result<(), ConsoleControlErr> {
    let (transmitter, receiver) = mpsc::channel();
    let _handle = notify_inputs::listen_and_notify_inputs(transmitter);

    let args: Vec<String> = env::args().collect();

    let x_len: usize = read_config(&args, "--x-len".to_string(), 10);
    let y_len: usize = read_config(&args, "--y-len".to_string(), 7);
    let seed: u64 = read_config(&args, "--seed".to_string(), 419);
    let print_mode: PrintMode = read_config(&args, "--mode".to_string(), PrintMode::PRETTY);
    let _ = conways_game::ConwaysGame::run_async(x_len, y_len, seed, print_mode, receiver)
        .join()
        .unwrap();

    return Ok(());
}

#[allow(dead_code)]
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
