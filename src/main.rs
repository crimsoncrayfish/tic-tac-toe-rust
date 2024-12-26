use coordination::service::CoordinatorService;
use std::env;
use utils::arg_helper::read_config;

pub mod utils {
    pub mod arg_helper;
    pub mod helper_macros;
    pub mod vec_t_writer;
}

pub mod coordination {
    pub mod service;
}
pub mod panel {
    pub mod command_enum;
    pub mod errors;
    pub mod panel;
    pub mod state;
}
pub mod rendering {
    pub mod colors;
    pub mod render_object;
    pub mod sprite;
}

pub mod shared {
    pub mod frame;
    pub mod usize2d;
}
pub mod handler {
    pub mod handle;
    pub mod handle_error;
    pub mod memory_handle;
    pub mod shared_handle;
    pub mod std_io_handle;
}

fn main() -> Result<(), SystemException> {
    let args: Vec<String> = env::args().collect();

    let _x_len: usize = read_config(&args, "--x-len".to_string(), 10);
    let _service = CoordinatorService::init();

    return Ok(());
}

#[derive(Debug)]
enum SystemException {
    _GameException,
    _InputReaderException,
    _CoordinatorException,
    _RedererException,
    _WindowsException,
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
