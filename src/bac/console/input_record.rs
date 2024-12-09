use std::fmt::Display;

use windows_sys::Win32::System::Console::{
    INPUT_RECORD, INPUT_RECORD_0, KEY_EVENT, KEY_EVENT_RECORD, KEY_EVENT_RECORD_0,
};

use crate::coordinate::Coord;

use super::errors::ConsoleControlErr;

#[derive(Clone, Copy)]
pub enum EventType {
    KeyEvent,
    MouseEvent,
}
impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::KeyEvent => write!(f, "key_pressed"),
            EventType::MouseEvent => write!(f, "mouse_thing"),
        }
    }
}
#[derive(Clone, Copy)]
pub struct InputRecord {
    pub event_type: EventType,
    pub event: InputEvent,
}

impl InputRecord {
    pub fn new_raw() -> INPUT_RECORD {
        let key_event_rec = KEY_EVENT_RECORD_0 { UnicodeChar: 0 };
        let key_event = KEY_EVENT_RECORD {
            bKeyDown: 0,
            wRepeatCount: 0,
            wVirtualKeyCode: 0,
            wVirtualScanCode: 0,
            uChar: key_event_rec,
            dwControlKeyState: 0,
        };
        let event = INPUT_RECORD_0 {
            KeyEvent: key_event,
        };
        INPUT_RECORD {
            EventType: 0,
            Event: event,
        }
    }
}

impl TryFrom<INPUT_RECORD> for InputRecord {
    type Error = ConsoleControlErr;
    // TODO: unit test this b#tch
    fn try_from(input_rec: INPUT_RECORD) -> Result<Self, ConsoleControlErr> {
        match input_rec.EventType as u32 {
            KEY_EVENT => Ok(InputRecord {
                event_type: EventType::KeyEvent,
                event: InputEvent {
                    key_event: KeyEvent {
                        command: char::from_u32(unsafe {
                            input_rec.Event.KeyEvent.uChar.UnicodeChar as u32
                        })
                        .unwrap(),
                        repreat_count: unsafe { input_rec.Event.KeyEvent.wRepeatCount },
                        is_down: unsafe { input_rec.Event.KeyEvent.bKeyDown } == 1,
                    },
                },
            }),
            _ => Err(ConsoleControlErr::WrongEventType),
        }
        // TODO: do this mapping right, i currently assume everything is a keyevent
    }
}

#[derive(Clone, Copy)]
pub union InputEvent {
    pub mouse_event: MouseEvent,
    pub key_event: KeyEvent,
    //pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    //pub MenuEvent: MENU_EVENT_RECORD,
    //pub FocusEvent: FOCUS_EVENT_RECORD,
}
#[derive(Clone, Copy)]
pub struct KeyEvent {
    pub command: char,
    pub repreat_count: u16,
    pub is_down: bool,
}

#[derive(Clone, Copy)]
pub struct MouseEvent {
    pub pos: Coord,
    pub button_state: u32,
    pub ctrl_key_state: u32,
    pub flags: u32,
}
