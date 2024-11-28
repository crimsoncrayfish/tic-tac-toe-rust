use std::str::FromStr;

use console::console_mode::{ConsoleControl, ConsoleControlErr};

pub mod console {
    pub mod console_mode;
}
pub mod conways_game;
pub mod conways_law;
pub mod input_handler;
pub mod terminal_formatter;

fn main() -> Result<(), ConsoleControlErr> {
    //let args: Vec<String> = env::args().collect();

    //let x_len: usize = read_config(&args, "--x-len".to_string(), 10);
    //let y_len: usize = read_config(&args, "--y-len".to_string(), 10);
    //let seed: u64 = read_config(&args, "--seed".to_string(), 10);
    //    let print_mode: PrintMode = read_config(&args, "--mode".to_string(), PrintMode::PRETTY);

    // TODO: read inputs
    // TODO: Print inputs
    // TODO: Print mouse location
    // TODO: toggle something on click

    let mut cm = ConsoleControl::init()?;
    let mode = cm.get_console_mode()?;
    print!("mode: {}\n", mode);
    match cm.set_uncooked_mode() {
        Ok(_) => println!("updated mode"),
        Err(e) => return Err(e),
    }
    let mode = cm.get_console_mode()?;
    print!("mode: {}\n", mode);
    match cm.set_cooked_mode() {
        Ok(_) => println!("updated mode"),
        Err(e) => return Err(e),
    }
    let mode = cm.get_console_mode()?;
    print!("mode: {}\n", mode);
    //let mut gs = conways_game::ConwaysGame::init(x_len, y_len, seed);
    //let duration = time::Duration::from_millis(1000);
    //gs.run(duration, print_mode);
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
