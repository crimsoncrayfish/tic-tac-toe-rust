use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum SharedErrors {
    BadCoordinate,
    OutOfBounds,
    ParsingFailure,
    BeforeStart,
    AfterEnd,
    None,
}
impl Display for SharedErrors {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
