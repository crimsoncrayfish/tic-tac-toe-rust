use std::{
    sync::mpsc::Receiver,
    thread::{spawn, JoinHandle},
};

use crate::{
    handler::handle::Handle,
    rendering::render_object::RenderObject,
    shared::{
        pixel_grid::{Frame, PixelGrid},
        shared_errors::SharedErrors,
        square::Square,
    },
};

use super::{command_enum::PanelCommandEnum, errors::PanelError, state::PanelState};

/// # Description
///
/// A panel describes a subsection of the available space on the screen in the terminal
///
/// Its size is described by a starting coordinate called `top_left` and an ending
/// coordinate called `bottom_right`
///
/// All data processed by this panel should render inside this area. Data is sent to the panel
/// through a sender in the form of list of renderable sprites `Vec<RenderObject>`
#[derive(Debug)]
pub struct Panel {
    _previous_frame: Frame,
    next_frame: Frame,
    area: Square, // TODO: DO i even want this since all objects are now relative to the panel
    frame_receiver: Receiver<Vec<RenderObject>>,
    command_receiver: Receiver<PanelCommandEnum>,
    state: PanelState,
    _out_handle: Box<dyn Handle>,
}
impl Panel {
    /// Initialize an instance of Window
    ///
    /// # Arguments
    ///
    /// * `size` - number of rows and columns in the grid as a usize
    /// * `top_left` - the top left most coordinate for the frame
    /// * `bottom_right` - the bottom right most coordinate for the frame
    /// * `frame_receiver` - receives the next frame to be printed
    /// * `command_receiver` - receives the commands for the window e.g. kill_process, resize_window
    ///
    /// #Returns
    ///
    /// An instance of a window
    ///
    /// # Examples
    ///
    /// ```
    /// let size = Usize2d::new(10, 69);
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let (_, frame_receiver) = channel();
    /// let (_, command_receiver) = channel();
    ///
    /// let window= Window::init(size, top_left, bottom_right, frame_receiver, command_receiver);
    /// ```
    fn init(
        area: Square,
        frame_receiver: Receiver<Vec<RenderObject>>,
        command_receiver: Receiver<PanelCommandEnum>,
        handle: Box<dyn Handle>,
    ) -> Result<Self, PanelError> {
        let new_state = Frame::default_with_size(area.width(), area.height());
        Ok(Panel {
            _previous_frame: new_state.clone(),
            next_frame: new_state.clone(),
            area,
            frame_receiver,
            command_receiver,
            state: PanelState::default(),
            _out_handle: handle,
        })
    }

    /// Run the window process
    ///
    /// # Examples
    ///
    /// ```
    /// let window= Window::init(...);
    /// window.run();
    /// ```
    pub fn run(&mut self) -> Result<(), PanelError> {
        loop {
            match self.command_receiver.try_recv() {
                Ok(cmd) => self.state.process_command(cmd),
                Err(_) => (),
            };

            if self.state.is_killed {
                break;
            }

            match self.frame_receiver.try_recv() {
                Ok(render_objects) => match self.process_objects(render_objects) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                Err(_) => {}
            };
            // TODO: self.render_frame();
            // TODO: self.push_frame();
        }
        Ok(())
    }

    /// Process received frame data
    ///
    /// # Examples
    ///
    /// ```
    /// let window= Window::init(...);
    /// let render_objects = vec![
    ///     RenderObject {
    ///         coordinate: Usize2d::default(),
    ///         sprite: vec![
    ///             "a".to_string(),
    ///             "b".to_string(),
    ///             "c".to_string(),
    ///         ]
    ///     }
    /// ];
    /// window.process_frame(render_objects);
    /// ```
    pub fn process_objects(
        &mut self,
        mut render_objects: Vec<RenderObject>,
    ) -> Result<(), PanelError> {
        render_objects.sort_by_key(|k| k.get_location().z);
        for object in render_objects {
            // TODO: This should write to the current frame in stead of the Handle
            let _was_written = self.write_object_next_frame(object);

            // TODO: log when an object was not written
        }
        Ok(())
    }

    /// Initialize and run on a new thread
    ///
    /// # Arguments
    ///
    /// * `size` - number of rows and columns in the grid as a usize
    /// * `top_left` - the top left most coordinate for the frame
    /// * `bottom_right` - the bottom right most coordinate for the frame
    /// * `frame_receiver` - receives the next frame to be printed
    /// * `command_receiver` - receives the commands for the window e.g. kill_process, resize_window
    ///
    /// # Returns
    ///
    /// A JoinHandle that can be used to block the main thread from exiting untill this process has
    /// been killed
    ///
    /// # Examples
    ///
    /// ```
    /// let size = Usize2d::new(10, 69);
    /// let top_left= Usize2d::new(0, 0);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let bottom_right= Usize2d::new(10, 69);
    /// let (_, frame_receiver) = channel();
    /// let (_, command_receiver) = channel();
    ///
    /// let handle= Window::init_run_async(size, top_left, bottom_right, frame_receiver, command_receiver);
    /// handle.join().unwrap();
    /// ```
    pub fn init_run_async(
        area: Square,
        frame_receiver: Receiver<Vec<RenderObject>>,
        command_receiver: Receiver<PanelCommandEnum>,
        handle: Box<dyn Handle>,
    ) -> Result<JoinHandle<()>, PanelError> {
        let mut w = Panel::init(area, frame_receiver, command_receiver, handle)?;
        let window_closure = move || {
            let _ = w.run();
        };
        Ok(spawn(window_closure))
    }
    /// Write a `RenderObject` to the handle
    ///
    /// # Arguments
    ///
    /// * `render_object` - an object to be rendered
    ///
    /// # Returns
    ///
    /// A result object indicating success
    ///
    /// # Example
    /// ```
    /// let panel = Panel::init();
    /// let render_object = RenderObject::default();
    /// let result = panel.write_object(render_object);
    /// assert!(result.is_ok());
    ///
    /// ```
    fn write_object_next_frame(&mut self, render_object: RenderObject) -> Result<(), PanelError> {
        if !self
            .area
            .overlaps_with(&render_object.get_area().move_by(self.area.get_top_left()))
        {
            return Err(PanelError::OutOfBounds);
        }
        let to_write: PixelGrid = render_object
            .get_content_to_write(self.area.move_to_origin())
            .map_err(|e| match e {
                SharedErrors::OutOfBounds => PanelError::OutOfBounds,
                _ => PanelError::BadRenderObject,
            })?;

        self.next_frame
            .write(to_write, render_object.get_location().as_2d());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{mpsc::channel, Arc, Mutex},
        thread::sleep,
        time::Duration,
    };

    use crate::{
        handler::{memory_handle::MemoryHandle, shared_handle::SharedHandle},
        panel::command_enum::PanelCommandEnum,
        rendering::{render_object::RenderObject, sprite::Sprite},
        shared::{
            pixel_grid::Frame,
            square::Square,
            usize2d::{Coord, Usize2d},
            usize3d::Coord3d,
        },
        vec_vec_u8_to_string,
    };

    use super::Panel;

    #[test]
    fn init() {
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let square = Square::new(top_left, bottom_right);
        let handle = Box::new(MemoryHandle::new());
        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let window = Panel::init(square, frame_receiver, command_receiver, handle);
        assert!(window.is_ok());
        let window = window.unwrap();
        let expected = Frame::default_with_size(11, 21);

        assert_eq!(
            window._previous_frame,
            expected,
            "Default initialization previous frame is wrong. Expected lenth: {}, Actual length: {}",
            expected.len(),
            window._previous_frame.len()
        );
        assert_eq!(
            window.next_frame,
            expected,
            "Default initialization for next frame is wrong. Expected lenth: {}, Actual length: {}",
            expected.len(),
            window.next_frame.len()
        );
    }

    #[test]
    fn run_and_kill() {
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let square = Square::new(top_left, bottom_right);

        let handle = Box::new(MemoryHandle::new());
        let (_, frame_receiver) = channel();
        let (command_sender, command_receiver) = channel();

        let handle = Panel::init_run_async(square, frame_receiver, command_receiver, handle);
        let result = command_sender.send(PanelCommandEnum::KillProcess);
        assert!(
            !result.is_err(),
            "There should be no bugs when sending the kill command"
        );
        sleep(Duration::from_millis(100));
        assert!(
            handle.unwrap().is_finished(),
            "The process should be completed due to the kill command"
        );
    }

    #[test]
    fn run_and_write() {
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let square = Square::new(top_left, bottom_right);

        let handle = Box::new(MemoryHandle::new());
        let (_frame_sender, frame_receiver) = channel();
        let (command_sender, command_receiver) = channel();

        let handle = Panel::init_run_async(square, frame_receiver, command_receiver, handle);

        // TODO: writer and write command

        let result = command_sender.send(PanelCommandEnum::KillProcess);
        assert!(
            !result.is_err(),
            "There should be no bugs when sending the kill command"
        );
        sleep(Duration::from_millis(100));
        assert!(
            handle.unwrap().is_finished(),
            "The process should be completed due to the kill command"
        );
    }
    #[test]
    fn write_object() {
        let test_cases = vec![
            (
                1,
                Usize2d::default(),
                Usize2d::new(10, 10),
                Coord3d::new(1, 1, 1),
                "           \n X X       \n  X        \n X X       \n           \n           \n           \n           \n           \n           \n           ",
                true
            ),
            (
                2,
                Usize2d::new(3, 5),
                Usize2d::new(10, 10),
                Coord3d::new(6, 3, 1),
                "        \n        \n        \n      X \n       X\n      X ",
                true
            ),
            (
                3,
                Usize2d::new(3, 5),
                Usize2d::new(10, 10),
                Coord3d::new(16, 3, 1),
                "",
               false 
            ),

        ];

        for (i, top_left, bottom_right, object_coordinate, expected, in_bounds) in test_cases {
            let square = Square::new(top_left, bottom_right);
            let mem_handle = Arc::new(Mutex::new(MemoryHandle::new()));

            let handle = SharedHandle::init(mem_handle.clone());
            let (_frame_sender, frame_receiver) = channel();
            let (_, command_receiver) = channel();

            let mut panel = Panel::init(square, frame_receiver, command_receiver, Box::new(handle))
                .expect("Failed to init the panel");

            let obj = RenderObject::new(Sprite::default(), object_coordinate);
            if in_bounds {
                let _ = panel
                    .write_object_next_frame(obj)
                    .expect(&format!("Test case {} failed to write object to handle", i)[..]);

                let actual_string = vec_vec_u8_to_string!(panel.next_frame.get_chars());

                assert_eq!(
                    actual_string, expected,
                    "Test case {} failed. Expected:\n{}\nGot:\n{}\n",
                    i, expected, actual_string
                );
            } else {
                let result = panel.write_object_next_frame(obj);
                assert!(result.is_err(), "object should be out of bounds");
            }
        }
    }
    #[test]
    fn write_objects() {
        let top_left = Coord::default();
        let bottom_right = Coord::new(10, 10);
        let square = Square::new(top_left, bottom_right);
        let mem_handle = Arc::new(Mutex::new(MemoryHandle::new()));

        let handle = SharedHandle::init(mem_handle.clone());
        let (_frame_sender, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let mut panel = Panel::init(square, frame_receiver, command_receiver, Box::new(handle))
            .expect("Failed to init the panel");
        let object_coordinate_0 = Coord3d::new(0, 1, 0);
        let obj_0 = RenderObject::new(Sprite::default(), object_coordinate_0);
        let object_coordinate_1 = Coord3d::new(1, 1, 1);
        let obj_1 = RenderObject::new(Sprite::default(), object_coordinate_1);
        let object_coordinate_2 = Coord3d::new(9, 6, 1);
        let obj_2 = RenderObject::new(Sprite::default(), object_coordinate_2);
        let object_coordinate_3 = Coord3d::new(3, 2, 2);
        let obj_3 = RenderObject::new(Sprite::default(), object_coordinate_3);
        let object_coordinate_4 = Coord3d::new(13, 12, 2);
        let obj_4 = RenderObject::new(Sprite::default(), object_coordinate_4);
        let _ = panel
            .process_objects(vec![obj_0, obj_1, obj_2, obj_3, obj_4])
            .expect(&format!("Failed to write object to handle")[..]);
        let actual_string = vec_vec_u8_to_string!(panel.next_frame.get_chars());
        let expected = "           \nXX X       \n  XX X     \nXX  X      \n   X X     \n           \n         X \n          X\n         X \n           \n           ";

        assert_eq!(
            actual_string, expected,
            "Expected:\n{}\nGot:\n{}\n",
            expected, actual_string
        )
    }
    fn get_shared_mem_handle_content(handle: Arc<Mutex<MemoryHandle>>) -> String {
        let locked_writer_result = handle.lock();
        let guard = locked_writer_result.unwrap();
        let locked = &*guard;
        let actual = locked.get_buffer_content();

        String::from_utf8_lossy(&actual).to_string()
    }
}
