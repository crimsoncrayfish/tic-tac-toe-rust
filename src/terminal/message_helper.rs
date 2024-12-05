use std::{collections::HashMap, fmt::Display};

use crate::{assert_r, coordinate::Coord};

use super::{
    formatter::{Terminal, TerminalColors},
    shared_writer::SharedWriter,
};

pub struct MessageHelper {
    characters: HashMap<char, [&'static str; 5]>,
    pub terminal: Terminal,
}
#[derive(Debug, PartialEq)]
pub enum MessageHelperErr {
    UnsupportedCharacters,
    MessageOutOfBounds,
    UnexpecterMessageLength,
}
impl Display for MessageHelperErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedCharacters => write!(f, "Unsupported character"),
            Self::MessageOutOfBounds => write!(f, "Message out of bounds"),
            Self::UnexpecterMessageLength => write!(f, "Unexpected message length"),
        }
    }
}
impl MessageHelper {
    pub fn print_around_centerpoint(
        &mut self,
        message: String,
        center: Coord,
    ) -> Result<(), MessageHelperErr> {
        let origin = self.calculate_origin(message.clone(), center);

        assert_r!(
            origin.x > 0 && origin.y > 0,
            MessageHelperErr::MessageOutOfBounds
        );

        self.print(message, origin)
    }

    pub fn calculate_origin(&self, message: String, center: Coord) -> Coord {
        let char_width = 7;
        let char_count = message.len() as u16;
        let width = (char_count as f32 / 2.0).floor() as u16;
        let mut x_start = 1;

        let width_to_subtract = width as u16 * char_width;
        if center.x > width_to_subtract {
            x_start = center.x - (width_to_subtract);
        }

        let mut y_start = 1;
        if center.y > 2 {
            y_start = center.y - 2;
        }
        Coord {
            x: x_start,
            y: y_start,
        }
    }

    pub fn print(&mut self, message: String, origin: Coord) -> Result<(), MessageHelperErr> {
        assert!(
            origin.x > 0 && origin.y > 0,
            "terminal locations are 1 indexed"
        );
        let chars: Vec<char> = message.chars().collect();
        for row in 0 as usize..5 {
            self.terminal
                .set_cursor_location(origin.x, row as u16 + origin.y);

            self.terminal.set_background(TerminalColors::White);
            self.terminal.set_foreground(TerminalColors::Red);
            for ind in 0..chars.len() {
                let current: char = match chars.get(ind) {
                    Some(ch) => ch.to_lowercase().next().unwrap(),
                    None => return Err(MessageHelperErr::UnexpecterMessageLength),
                };
                let printable_char: [&'static str; 5] = match self.characters.get(&current) {
                    Some(printable) => *printable,
                    None => return Err(MessageHelperErr::UnsupportedCharacters),
                };
                let printable_row: &str = printable_char[row];
                self.terminal.write(printable_row.to_string());
            }
            self.terminal.flush();
        }
        Ok(())
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

    use super::{MessageHelper, MessageHelperErr};

    #[test]
    fn print_at_origin() {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let writer = SharedWriter::init(buffer.clone());
        let mut helper = MessageHelper::init(writer);

        match helper.print("hello".to_string(), crate::coordinate::Coord { x: 1, y: 1 }) {
            Ok(_) => (),
            Err(_) => assert!(false, "Should not have an error at this point"),
        };

        let unwrapped = buffer.lock().unwrap();
        let result = String::from_utf8_lossy(&unwrapped);
        let expected ="\u{1b}[1;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █  █████  █      █       ███  \u{1b}[2;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █  █      █      █      █   █ \u{1b}[3;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █████  ████   █      █      █   █ \u{1b}[4;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █  █      █      █      █   █ \u{1b}[5;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █  █████  █████  █████   ███  ";
        assert_eq!(
            expected, result,
            "The output written should look like a chunky version of the string \"hello\""
        )
    }
    #[test]
    fn print_at_location() {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let writer = SharedWriter::init(buffer.clone());
        let mut helper = MessageHelper::init(writer);
        match helper.print(
            "world".to_string(),
            crate::coordinate::Coord { x: 10, y: 18 },
        ) {
            Ok(_) => (),
            Err(_) => assert!(false, "Should not have an error at this point"),
        };

        let unwrapped = buffer.lock().unwrap();
        let result = String::from_utf8_lossy(&unwrapped);
        let expected = "\u{1b}[18;10H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █   ███   ████   █      ███   \u{1b}[19;10H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █  █   █  █   █  █      █  █  \u{1b}[20;10H\u{1b}[48;5;231m\u{1b}[38;5;160m █ █ █  █   █  ████   █      █   █ \u{1b}[21;10H\u{1b}[48;5;231m\u{1b}[38;5;160m ██ ██  █   █  █  █   █      █  █  \u{1b}[22;10H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █   ███   █   █  █████  ███   ";
        assert_eq!(
            expected, result,
            "The output written should look like a chunky version of the string \"world\" but shifter 17 positions down and 9 positions to the right"
        )
    }
    #[test]
    fn print_around_location() {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let writer = SharedWriter::init(buffer.clone());
        let mut helper = MessageHelper::init(writer);

        match helper.print_around_centerpoint(
            "what even".to_string(),
            crate::coordinate::Coord { x: 10, y: 18 },
        ) {
            Ok(_) => (),
            Err(_) => assert!(false, "Should not have an error at this point"),
        };
        let unwrapped = buffer.lock().unwrap();
        let result = String::from_utf8_lossy(&unwrapped);
        let expected = "\u{1b}[16;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █  █   █   ███   █████         █████  █   █  █████  █   █ \u{1b}[17;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █  █   █  █   █    █           █      █   █  █      ██  █ \u{1b}[18;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █ █ █  █████  █████    █           ████   █   █  ████   █ █ █ \u{1b}[19;1H\u{1b}[48;5;231m\u{1b}[38;5;160m ██ ██  █   █  █   █    █           █       █ █   █      █  ██ \u{1b}[20;1H\u{1b}[48;5;231m\u{1b}[38;5;160m █   █  █   █  █   █    █           █████    █    █████  █   █ ";
        assert_eq!(
            expected, result,
            "The output written should look like a chunky version of the string \"what even\" but printed around the centerpoint x = 10 and y = 18"
        )
    }
    #[test]
    fn print_unsupported_characters() {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let writer = SharedWriter::init(buffer.clone());
        let mut helper = MessageHelper::init(writer);

        match helper.print_around_centerpoint(
            "what even?".to_string(),
            crate::coordinate::Coord { x: 10, y: 18 },
        ) {
            Ok(_) => assert!(false, "Should have an error at this point"),
            Err(e) => assert_eq!(e, MessageHelperErr::UnsupportedCharacters),
        };
    }
}
