use std::{
    env,
    sync::mpsc::{self},
    thread::sleep,
    time::Duration,
};

use arg_helper::read_config;
use console::errors::ConsoleControlErr;
use console::notify_inputs;
use conway::{conways_game, print_mode::PrintMode};

pub mod console {
    pub mod console_control;
    pub mod errors;
    pub mod input_record;
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
pub mod arg_helper;
pub mod coordinate;
pub mod terminal {
    pub mod formatter;
    pub mod message_helper;
    pub mod shared_writer;
}

fn main() -> Result<(), ConsoleControlErr> {
    sleep(Duration::from_secs(3));
    let (transmitter, receiver) = mpsc::channel();
    let _handle = notify_inputs::listen_and_notify_key_inputs(transmitter);

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
