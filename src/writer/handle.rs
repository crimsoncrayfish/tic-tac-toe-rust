use std::io::Write;

pub trait Handle: Write + Send {}
