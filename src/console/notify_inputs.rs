use std::{
    sync::mpsc::Sender,
    thread::{spawn, JoinHandle},
};

use super::{
    console_control::{ConsoleCommand, ConsoleControl},
    errors::ConsoleControlErr,
    mode::ConsoleMode,
};

pub fn listen_and_notify_inputs(
    sender: Sender<ConsoleCommand>,
) -> JoinHandle<Result<(), ConsoleControlErr>> {
    let read_input_closure = move || -> Result<(), ConsoleControlErr> {
        // TODO: Handle this err
        let cm = ConsoleControl::init()?;
        match cm.set_mode(ConsoleMode::Uncooked) {
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

        match cm.set_mode(ConsoleMode::Cooked) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    };
    spawn(read_input_closure)
}
