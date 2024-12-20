use coordination::service::CoordinatorService;
use std::env;
use utils::arg_helper::read_config;

pub mod utils {
    pub mod arg_helper;
    pub mod helper_macros;
}
pub mod coordination {
    pub mod service;
}
pub mod windows {
    pub mod command_enum;
    pub mod errors;
    pub mod state;
    pub mod window;
}
pub mod rendering {
    pub mod colors;
}

pub mod shared {
    pub mod frame;
    pub mod usize2d;
}

fn main() -> Result<(), SystemException> {
    let args: Vec<String> = env::args().collect();

    let _x_len: usize = read_config(&args, "--x-len".to_string(), 10);
    let _service = CoordinatorService::init();

    return Ok(());
}

#[derive(Debug)]
enum SystemException {
    GameException,
    InputReaderException,
    CoordinatorException,
    RedererException,
    WindowsException,
}

/// Placeholder
///
/// # Arguments
///
/// * `my_input` - my input
///
/// #Returns
///
/// Does placeholder stuff
///
/// # Examples
///
/// ```
/// let x = placeholder()
/// ```
pub fn placeholder() {}
