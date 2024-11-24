use core::time;

pub mod conways_game;
pub mod conways_law;
pub mod terminal_formatter;

fn main() {
    let mut gs = conways_game::ConwaysGame::init(10, 10, 10);
    let duration = time::Duration::from_millis(1000);
    gs.run(duration);
}
