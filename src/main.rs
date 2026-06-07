use coordination::service::CoordinatorService;
use std::{
    env,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::Duration,
};
use utils::arg_helper::read_config;

use crate::{
    handler::{
        handle::Handle, handle_error::HandleError, shared_handle::SharedHandle,
        std_io_handle::StdIOHandle,
    },
    panel::actual::Panel,
    rendering::{colors::TerminalColors, render_object::RenderObject, sprite::Sprite},
    shared::{
        pixel_grid::PixelGrid,
        square::Square,
        usize2d::{Coord, Usize2d},
        usize3d::Coord3d,
    },
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

    let mut shared_handle =
        SharedHandle::<StdIOHandle>::init(Arc::new(Mutex::new(Box::new(StdIOHandle::default()))));
    if shared_handle
        .write_to_location(b"\x1b[2J\x1b[H", Coord::new(0, 0))
        .is_err()
    {
        return Err(SystemException::_Windows);
    };
    if shared_handle
        .write_to_location(b"\x1b[?25l", Coord::new(0, 0))
        .is_err()
    {
        return Err(SystemException::_Windows);
    };
    if fill_terminal(&mut shared_handle, 200, 200, b'.').is_err() {
        return Err(SystemException::_Windows);
    }
    let (frame_sender1, frame_receiver1) = channel();
    let (command_sender1, command_receiver1) = channel();
    let (shutdown_sender1, shutdown_receiver1) = channel();

    let (frame_sender2, frame_receiver2) = channel();
    let (command_sender2, command_receiver2) = channel();
    let (shutdown_sender2, shutdown_receiver2) = channel();

    const PANEL_W: usize = 40;
    const PANEL_H: usize = 20;

    let panel1 = match Panel::init_run_async(
        Square::new(Usize2d::new(0, 0), Usize2d::new(PANEL_W, PANEL_H)),
        frame_receiver1,
        command_receiver1,
        shared_handle.clone(),
    ) {
        Ok(h) => h,
        Err(_) => return Err(SystemException::Panel),
    };

    let panel2 = match Panel::init_run_async(
        Square::new(
            Usize2d::new(45, 20),
            Usize2d::new(45 + PANEL_W, 20 + PANEL_H),
        ),
        frame_receiver2,
        command_receiver2,
        shared_handle.clone(),
    ) {
        Ok(h) => h,
        Err(_) => return Err(SystemException::Panel),
    };

    let producer1 = thread::spawn(move || {
        let mut c = Coord3d::default();
        let mut dir = Dir3d::new(1, 1, 1);

        loop {
            let _ = frame_sender1.send(construct_test_frame(c));

            if c.x >= PANEL_W {
                dir.x = -1;
            } else if c.x == 0 {
                dir.x = 1;
            }

            if c.y >= PANEL_H {
                dir.y = -1;
            } else if c.y == 0 {
                dir.y = 1;
            }

            c = c.add_signed(dir.x, dir.y, dir.z);
            thread::sleep(Duration::from_millis(100));

            if shutdown_receiver1.try_recv().is_ok() {
                return;
            }
        }
    });

    let producer2 = thread::spawn(move || {
        let mut c = Coord3d::new(10, 5, 0);
        let mut dir = Dir3d::new(1, -1, 1);

        loop {
            let _ = frame_sender2.send(construct_test_frame(c));

            if c.x >= PANEL_W {
                dir.x = -1;
            } else if c.x == 0 {
                dir.x = 1;
            }

            if c.y >= PANEL_H {
                dir.y = -1;
            } else if c.y == 0 {
                dir.y = 1;
            }

            c = c.add_signed(dir.x, dir.y, dir.z);
            thread::sleep(Duration::from_millis(140));

            if shutdown_receiver2.try_recv().is_ok() {
                return;
            }
        }
    });

    thread::sleep(Duration::from_secs(30));

    let _ = command_sender1.send(panel::command_enum::PanelCommandEnum::KillProcess);
    let _ = command_sender2.send(panel::command_enum::PanelCommandEnum::KillProcess);

    let _ = shutdown_sender1.send(panel::command_enum::PanelCommandEnum::KillProcess);
    let _ = shutdown_sender2.send(panel::command_enum::PanelCommandEnum::KillProcess);

    match panel1.join() {
        Ok(_) => (),
        Err(_) => return Err(SystemException::Join),
    };

    match panel2.join() {
        Ok(_) => (),
        Err(_) => return Err(SystemException::Join),
    };

    match producer1.join() {
        Ok(_) => (),
        Err(_) => return Err(SystemException::Join),
    };

    match producer2.join() {
        Ok(_) => (),
        Err(_) => return Err(SystemException::Join),
    };

    if shared_handle
        .write_to_location(b"\x1b[?25h", Coord::new(0, 0))
        .is_err()
    {
        return Err(SystemException::_Windows);
    };
    if shared_handle
        .write_to_location(b"\x1b[2J\x1b[H", Coord::new(0, 0))
        .is_err()
    {
        return Err(SystemException::_Windows);
    };
    Ok(())
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Dir3d {
    x: isize,
    y: isize,
    z: isize,
}
impl Dir3d {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}
fn construct_test_frame(c: Coord3d) -> Vec<RenderObject> {
    // Define the 5x3 ASCII lines: "(^_^)"
    let row_1 = vec![b'(', b'^', b'_', b'^', b')'];
    let row_2 = vec![b'(', b'^', b'_', b'^', b')'];
    let row_3 = vec![b'(', b'^', b'_', b'^', b')'];

    let simple_grid = PixelGrid::new(
        vec![row_1, row_2, row_3],
        vec![
            vec![TerminalColors::White; 5],
            vec![TerminalColors::Black; 5],
            vec![TerminalColors::Red; 5],
        ],
        vec![
            vec![TerminalColors::LightGreen; 5],
            vec![TerminalColors::Red; 5],
            vec![TerminalColors::HotPink; 5],
        ],
    );

    let smiley_sprite = Sprite::new_from_grid("smiley_face".to_string(), 5, 3, simple_grid);

    let render_object = RenderObject::new(smiley_sprite, c);

    vec![render_object]
}
fn fill_terminal<H: Handle>(
    handle: &mut H,
    width: usize,
    height: usize,
    fill: u8,
) -> Result<(), HandleError> {
    handle.write_to_location(b"\x1b[2J\x1b[H", Coord::new(0, 0))?;

    let line = vec![fill; width];

    for y in 0..height {
        handle.write_to_location(&line, Coord::new(0, y))?;
    }

    Ok(())
}
#[derive(Debug)]
enum SystemException {
    _Game,
    _InputReader,
    _Coordinator,
    Panel,
    _Rederer,
    _Windows,
    Join,
    _Send,
}
