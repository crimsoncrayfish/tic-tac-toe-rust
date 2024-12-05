use std::collections::HashMap;

use crate::coordinate::Coord;

use super::{
    formatter::{Terminal, TerminalColors},
    shared_writer::SharedWriter,
};

pub struct MessageHelper {
    characters: HashMap<char, [&'static str; 5]>,
    pub terminal: Terminal,
}
impl MessageHelper {
    pub fn print_around_centerpoint(&mut self, message: String, center: Coord) {
        let char_width = 7;
        let char_count = message.len() as u16;
        let width = (char_count as f32 / 2.0).floor() as u16;
        let mut x_start = 0;

        if center.x > width {
            x_start = center.x - (width as u16 * char_width);
        }
        let location = Coord {
            x: x_start,
            y: center.y - 2,
        };
        self.print(message, location);
    }
    pub fn print(&mut self, message: String, location: Coord) {
        let chars: Vec<char> = message.chars().collect();
        for row in 0 as usize..5 {
            println!("setting cursor");
            self.terminal
                .set_cursor_location(location.x, row as u16 + location.y);

            println!("setting background colors");
            self.terminal.set_background(TerminalColors::White);
            println!("setting foreground colors");
            self.terminal.set_foreground(TerminalColors::Red);
            for ind in 0..chars.len() {
                let current: char = match chars.get(ind) {
                    Some(ch) => ch.to_lowercase().next().unwrap(),
                    None => {
                        assert!(false, "This should never happen as the index is calculated");
                        continue;
                    }
                };
                let printable_char: [&'static str; 5] = match self.characters.get(&current) {
                    Some(printable) => *printable,
                    None => {
                        assert!(false, "Character not supported");
                        continue;
                    }
                };
                let printable_row: &str = printable_char[row];
                println!("printing chars:{:?}", printable_row);
                self.terminal.write(printable_row.to_string());
            }
            println!("flush");
            self.terminal.flush();
        }
    }
    pub fn init(w: SharedWriter) -> Self {
        let mut characters: HashMap<char, [&'static str; 5]> = HashMap::new();
        characters.insert(' ', SPACE);
        characters.insert('a', A);
        characters.insert('b', B);
        characters.insert('c', C);
        characters.insert('d', D);
        characters.insert('e', E);
        characters.insert('f', F);
        characters.insert('g', G);
        characters.insert('h', H);
        characters.insert('i', I);
        characters.insert('j', J);
        characters.insert('k', K);
        characters.insert('l', L);
        characters.insert('m', M);
        characters.insert('n', N);
        characters.insert('o', O);
        characters.insert('p', P);
        characters.insert('q', Q);
        characters.insert('r', R);
        characters.insert('s', S);
        characters.insert('t', T);
        characters.insert('u', U);
        characters.insert('v', V);
        characters.insert('w', W);
        characters.insert('x', X);
        characters.insert('y', Y);
        characters.insert('z', Z);
        characters.insert('0', NUM_0);
        characters.insert('1', NUM_1);
        characters.insert('2', NUM_2);
        characters.insert('3', NUM_3);
        characters.insert('4', NUM_4);
        characters.insert('5', NUM_5);
        characters.insert('6', NUM_6);
        characters.insert('7', NUM_7);
        characters.insert('8', NUM_8);
        characters.insert('9', NUM_9);
        print!("Message Helper Setup");
        MessageHelper {
            characters,
            terminal: Terminal::init(w),
        }
    }
}
const A: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    " █████ ", //
    " █   █ ", //
    " █   █ ", //
];
const B: [&'static str; 5] = [
    " ████  ", //
    " █   █ ", //
    " ████  ", //
    " █   █ ", //
    " ████  ", //
];
const C: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    " █     ", //
    " █   █ ", //
    "  ███  ", //
];
const D: [&'static str; 5] = [
    " ███   ", //
    " █  █  ", //
    " █   █ ", //
    " █  █  ", //
    " ███   ", //
];
const E: [&'static str; 5] = [
    " █████ ", //
    " █     ", //
    " ████  ", //
    " █     ", //
    " █████ ", //
];
const F: [&'static str; 5] = [
    " █████ ", //
    " █     ", //
    " ████  ", //
    " █     ", //
    " █     ", //
];
const G: [&'static str; 5] = [
    "  ████ ", //
    " █     ", //
    " █  ██ ", //
    " █   █ ", //
    "  ███  ", //
];
const H: [&'static str; 5] = [
    " █   █ ", //
    " █   █ ", //
    " █████ ", //
    " █   █ ", //
    " █   █ ", //
];
const I: [&'static str; 5] = [
    " █████ ", //
    "   █   ", //
    "   █   ", //
    "   █   ", //
    " █████ ", //
];
const J: [&'static str; 5] = [
    " █████ ", //
    "     █ ", //
    "     █ ", //
    " █   █ ", //
    "  ███  ", //
];
const K: [&'static str; 5] = [
    " █   █ ", //
    " █  █  ", //
    " ███   ", //
    " █  █  ", //
    " █   █ ", //
];
const L: [&'static str; 5] = [
    " █     ", //
    " █     ", //
    " █     ", //
    " █     ", //
    " █████ ", //
];
const M: [&'static str; 5] = [
    " █   █ ", //
    " ██ ██ ", //
    " █ █ █ ", //
    " █   █ ", //
    " █   █ ", //
];
const N: [&'static str; 5] = [
    " █   █ ", //
    " ██  █ ", //
    " █ █ █ ", //
    " █  ██ ", //
    " █   █ ", //
];
const O: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    " █   █ ", //
    " █   █ ", //
    "  ███  ", //
];
const P: [&'static str; 5] = [
    " ████  ", //
    " █   █ ", //
    " ████  ", //
    " █     ", //
    " █     ", //
];
const Q: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    " █   █ ", //
    " █  ██ ", //
    "  ████ ", //
];
const R: [&'static str; 5] = [
    " ████  ", //
    " █   █ ", //
    " ████  ", //
    " █  █  ", //
    " █   █ ", //
];
const S: [&'static str; 5] = [
    "  ████ ", //
    " █     ", //
    "  ███  ", //
    "     █ ", //
    " ████  ", //
];
const T: [&'static str; 5] = [
    " █████ ", //
    "   █   ", //
    "   █   ", //
    "   █   ", //
    "   █   ", //
];
const U: [&'static str; 5] = [
    " █   █ ", //
    " █   █ ", //
    " █   █ ", //
    " █   █ ", //
    "  ███  ", //
];
const V: [&'static str; 5] = [
    " █   █ ", //
    " █   █ ", //
    " █   █ ", //
    "  █ █  ", //
    "   █   ", //
];
const W: [&'static str; 5] = [
    " █   █ ", //
    " █   █ ", //
    " █ █ █ ", //
    " ██ ██ ", //
    " █   █ ", //
];
const X: [&'static str; 5] = [
    " █   █ ", //
    "  █ █  ", //
    "   █   ", //
    "  █ █  ", //
    " █   █ ", //
];
const Y: [&'static str; 5] = [
    " █   █ ", //
    "  █ █  ", //
    "   █   ", //
    "   █   ", //
    "   █   ", //
];
const Z: [&'static str; 5] = [
    " █████ ", //
    "     █ ", //
    "   █   ", //
    " █     ", //
    " █████ ", //
];
const NUM_0: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    " █   █ ", //
    " █   █ ", //
    "  ███  ", //
];
const NUM_1: [&'static str; 5] = [
    "   █   ", //
    "  ██   ", //
    "   █   ", //
    "   █   ", //
    "  ███  ", //
];
const NUM_2: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    "    █  ", //
    "   █   ", //
    " █████ ", //
];
const NUM_3: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    "    ██ ", //
    " █   █ ", //
    "  ███  ", //
];
const NUM_4: [&'static str; 5] = [
    "    ██ ", //
    "   █ █ ", //
    "  █  █ ", //
    " █████ ", //
    "     █ ", //
];
const NUM_5: [&'static str; 5] = [
    " █████ ", //
    " █     ", //
    " ████  ", //
    "     █ ", //
    " ████  ", //
];
const NUM_6: [&'static str; 5] = [
    "  ███  ", //
    " █     ", //
    " ████  ", //
    " █   █ ", //
    "  ███  ", //
];
const NUM_7: [&'static str; 5] = [
    " █████ ", //
    "     █ ", //
    "    █  ", //
    "   █   ", //
    "  █    ", //
];
const NUM_8: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    "  ███  ", //
    " █   █ ", //
    "  ███  ", //
];
const NUM_9: [&'static str; 5] = [
    "  ███  ", //
    " █   █ ", //
    "  ████ ", //
    "     █ ", //
    "  ███  ", //
];
const SPACE: [&'static str; 5] = [
    "       ", //
    "       ", //
    "       ", //
    "       ", //
    "       ", //
];

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::terminal::shared_writer::SharedWriter;

    use super::MessageHelper;

    #[test]
    fn print_at() {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let writer = SharedWriter::init(buffer.clone());
        let mut helper = MessageHelper::init(writer);

        helper.print("hello".to_string(), crate::coordinate::Coord { x: 0, y: 0 });

        let unwrapped = buffer.lock().unwrap();
        let result = String::from_utf8_lossy(&unwrapped);

        print!("OUTPUT:\"{}\"\n", result);
    }
}
