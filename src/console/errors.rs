use core::fmt::Display;

#[derive(Debug)]
pub enum ConsoleControlErr {
    NoHandle,
    NoModeResponse,
    ModeTypeUnknown,
    SetModeFailed,
    NoInputRead,
    WrongEventType,
}
impl Display for ConsoleControlErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed...")
    }
}
