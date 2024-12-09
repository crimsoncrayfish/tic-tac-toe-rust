use core::fmt::Display;
use core::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
pub enum PrintMode {
    PRETTY,
    DEBUG,
}
impl Display for PrintMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintMode::DEBUG => write!(f, "Debug"),
            PrintMode::PRETTY => write!(f, "Pretty"),
        }
    }
}
impl FromStr for PrintMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pretty" | "PRETTY" => Ok(PrintMode::PRETTY),
            "debug" | "DEBUG" => Ok(PrintMode::DEBUG),
            _ => Err(format!("`{}` is not a valid mode", s)),
        }
    }
}
