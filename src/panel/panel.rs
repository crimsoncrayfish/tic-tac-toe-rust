use std::{
    sync::mpsc::Receiver,
    thread::{spawn, JoinHandle},
};

use crate::{
    handler::handle::Handle,
    rendering::render_object::RenderObject,
    shared::{frame::Pixel, square::Square},
};

use super::{command_enum::PanelCommandEnum, errors::PanelException, state::PanelState};

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
    _previous_frame: Vec<Vec<Pixel>>,
    _next_frame: Vec<Vec<Pixel>>,
    _area: Square,
    frame_receiver: Receiver<Vec<RenderObject>>,
    command_receiver: Receiver<PanelCommandEnum>,
    state: PanelState,
    _handle: Box<dyn Handle>,
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
    ) -> Result<Self, PanelException> {
        let new_state = vec![vec![Pixel::default(); area.width()]; area.height()];
        Ok(Panel {
            _previous_frame: new_state.clone(),
            _next_frame: new_state.clone(),
            _area: area,
            frame_receiver,
            command_receiver,
            state: PanelState::default(),
            _handle: handle,
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
    pub fn run(&mut self) -> Result<(), PanelException> {
        loop {
            match self.command_receiver.try_recv() {
                Ok(cmd) => self.state.process_command(cmd),
                Err(_) => (),
            };

            if self.state.is_killed {
                break;
            }

            match self.frame_receiver.try_recv() {
                Ok(render_objects) => match self.process_frame(render_objects) {
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
    pub fn process_frame(
        &mut self,
        _render_objects: Vec<RenderObject>,
    ) -> Result<(), PanelException> {
        // TODO: write the objects to the panel
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
    ) -> Result<JoinHandle<()>, PanelException> {
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
    fn write_object(self, _render_object: RenderObject) -> Result<(), PanelException> {
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
        panel::{command_enum::PanelCommandEnum, errors::PanelException},
        rendering::{render_object::RenderObject, sprite::Sprite},
        shared::{
            frame::Pixel,
            square::Square,
            usize2d::{Coord, Usize2d},
        },
    };

    use super::Panel;

    #[test]
    fn init() {
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let result = Square::new(top_left, bottom_right);
        let square = result.unwrap();
        let handle = Box::new(MemoryHandle::new());
        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let window = Panel::init(square, frame_receiver, command_receiver, handle);
        assert!(window.is_ok());
        let window = window.unwrap();
        let expected = vec![vec![Pixel::default(); 11]; 21];

        assert_eq!(
            window._previous_frame,
            expected,
            "Default initialization previous frame is wrong. Expected lenth: {}, Actual length: {}",
            expected.len(),
            window._previous_frame.len()
        );
        assert_eq!(
            window._next_frame,
            expected,
            "Default initialization for next frame is wrong. Expected lenth: {}, Actual length: {}",
            expected.len(),
            window._next_frame.len()
        );
    }

    #[test]
    fn run_and_kill() {
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let result = Square::new(top_left, bottom_right);
        let square = result.unwrap();

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
        let result = Square::new(top_left, bottom_right);
        let square = result.unwrap();

        let handle = Box::new(MemoryHandle::new());
        let (_frame_sender, frame_receiver) = channel();
        let (command_sender, command_receiver) = channel();

        let handle = Panel::init_run_async(square, frame_receiver, command_receiver, handle);

        //TODO: writer and write command

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
    pub fn write_object() {
        let top_left = Usize2d::new(3, 4);
        let bottom_right = Usize2d::new(13, 8);
        let result = Square::new(top_left, bottom_right);
        let square = result.unwrap();
        let mem_handle = Arc::new(Mutex::new(MemoryHandle::new()));

        let handle = SharedHandle::init(mem_handle.clone());
        let (_frame_sender, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let panel = Panel::init(square, frame_receiver, command_receiver, Box::new(handle))
            .expect("Failed to init the panel");

        let obj = RenderObject::new(Sprite::default(), Coord::new(10, 6));
        let _ = panel
            .write_object(obj)
            .expect("Failed to write object to handle");

        let actual_string = get_shared_mem_handle_content(mem_handle.clone());
        let expected = "\n\n\n\n\n\n   X X\n    X \n   X X";
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
