use core::time;
use std::thread::sleep;

pub mod conways_game;
pub mod conways_law;

fn main() {
    hide_cursor();
    let mut gs = conways_game::ConwaysGame::init(10, 10, 10);

    let duration = time::Duration::from_millis(1000);
    loop {
        sleep(duration);
        reset_terminal();
        gs.next();
        gs.debug_print();
    }
}

pub fn reset_terminal() {
    println!("\x1Bc");
}
pub fn hide_cursor() {
    println!("\x1B[?25h");
}
pub fn show_cursor() {
    println!("\x1B[?25l");
}
