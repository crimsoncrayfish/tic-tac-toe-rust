use std::{
    sync::mpsc::Sender,
    thread::{spawn, JoinHandle},
};

use super::{
    console_control::ConsoleControl, errors::ConsoleControlErr, input_record::KeyEvent,
    mode::ConsoleMode,
};

pub fn listen_and_notify_key_inputs(
    sender: Sender<KeyEvent>,
) -> JoinHandle<Result<(), ConsoleControlErr>> {
    let read_input_closure = move || -> Result<(), ConsoleControlErr> {
        let cm = ConsoleControl::init()?;
        match cm.set_mode(ConsoleMode::Uncooked) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        loop {
            // TODO: Handle this err
            match cm.read_console_input() {
                Ok(ch) => {
                    match sender.send(ch.clone()) {
                        Ok(_) => (),
                        Err(e) => {
                            // TODO: this is bad
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
                Err(_) => {
                    // TODO: WHAT DO I DO
                    // 1. Maybe write errors to a file since i dont have the terminal?
                    // 2. Maybe send errors to the queue?
                }
            };
        }

        match cm.set_mode(ConsoleMode::Cooked) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    };
    spawn(read_input_closure)
}
