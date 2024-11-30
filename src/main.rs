use core::time;
use std::{
    env,
    str::FromStr,
    sync::mpsc::{self, Receiver, Sender},
    thread::{spawn, JoinHandle},
};

use console::console_control::{ConsoleCommand, ConsoleControl, ConsoleControlErr};
use conways_game::PrintMode;

pub mod console {
    pub mod console_control;
}
pub mod conways_game;
pub mod conways_law;
pub mod coordinate;
pub mod input_handler;
pub mod terminal_formatter;

fn main() -> Result<(), ConsoleControlErr> {
    let (transmitter, receiver) = mpsc::channel();
    let _handle = spawn_input_thread(transmitter);

    let args: Vec<String> = env::args().collect();

    let x_len: usize = read_config(&args, "--x-len".to_string(), 10);
    let y_len: usize = read_config(&args, "--y-len".to_string(), 7);
    let seed: u64 = read_config(&args, "--seed".to_string(), 419);
    let print_mode: PrintMode = read_config(&args, "--mode".to_string(), PrintMode::PRETTY);
    let _ = spawn_game_thread(x_len, y_len, seed, print_mode, receiver)
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
fn spawn_input_thread(sender: Sender<ConsoleCommand>) -> JoinHandle<Result<(), ConsoleControlErr>> {
    let read_input_closure = move || -> Result<(), ConsoleControlErr> {
        // TODO: Handle this err
        let cm = ConsoleControl::init()?;
        match cm.set_uncooked_mode() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        loop {
            // TODO: Handle this err
            let ch = cm.read_console_input()?;

            match sender.send(ch.clone()) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!(
                        "Exception occurred when sending command on channel with error: {}",
                        e
                    );
                    break;
                }
            }

            if ch.command == 'q' {
                break;
            }
        }

        match cm.set_cooked_mode() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    };
    spawn(read_input_closure)
}
fn spawn_game_thread(
    x_len: usize,
    y_len: usize,
    seed: u64,
    print_mode: PrintMode,
    receiver: Receiver<ConsoleCommand>,
) -> JoinHandle<()> {
    let game_closure = move || {
        let mut gs = conways_game::ConwaysGame::init(x_len, y_len, seed, print_mode, receiver);
        let duration = time::Duration::from_millis(1000);
        gs.run(duration);
    };
    spawn(game_closure)
}
