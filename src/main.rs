use coordination::service::CoordinatorService;
use std::{
    env,
    sync::{mpsc::channel, Arc, Mutex},
};
use utils::arg_helper::read_config;

use crate::{
    handler::{shared_handle::SharedHandle, std_io_handle::StdIOHandle},
    panel::actual::Panel,
    shared::{square::Square, usize2d::Usize2d},
};

pub mod utils {
    pub mod arg_helper;
    pub mod helper_macros;
    pub mod vec_t_writer;
    pub mod vec_vec_helper;
}

pub mod coordination {
    pub mod service;
}
pub mod panel {
    pub mod actual;
    pub mod command_enum;
    pub mod errors;
    pub mod state;
}
pub mod rendering {
    pub mod colors;
    pub mod render_object;
    pub mod sprite;
}

pub mod shared {
    pub mod pixel_grid;
    pub mod shared_errors;
    pub mod square;
    pub mod usize2d;
    pub mod usize3d;
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
    let _service = CoordinatorService::<SharedHandle<StdIOHandle>>::init();
    let handle = handler::shared_handle::SharedHandle::<StdIOHandle>::init(Arc::new(Mutex::new(
        Box::new(StdIOHandle::default()),
    )));
    let (_, frame_receiver) = channel();
    let (_, command_receiver) = channel();
    let panel = Panel::init(
        Square::new(Usize2d::new(0, 0), Usize2d::new(10, 10)),
        frame_receiver,
        command_receiver,
        handle,
    );
    if let Ok(mut p) = panel {
        _ = p.run();
    }

    Ok(())
}

#[derive(Debug)]
enum SystemException {
    _Game,
    _InputReader,
    _Coordinator,
    _Rederer,
    _Windows,
}
