use std::{
    sync::mpsc::Receiver,
    thread::{spawn, JoinHandle},
};

use crate::{
    assert_r,
    rendering::render_object::RenderObject,
    shared::{frame::Pixel, usize2d::Usize2d},
    writer::handle::Handle,
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
    previous_frame: Vec<Vec<Pixel>>,
    next_frame: Vec<Vec<Pixel>>,
    top_left: Usize2d,
    bottom_right: Usize2d,
    frame_receiver: Receiver<Vec<RenderObject>>,
    command_receiver: Receiver<PanelCommandEnum>,
    state: PanelState,
    writer: Box<dyn Handle>,
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
        size: Usize2d,
        top_left: Usize2d,
        bottom_right: Usize2d,
        frame_receiver: Receiver<Vec<RenderObject>>,
        command_receiver: Receiver<PanelCommandEnum>,
        handle: Box<dyn Handle>,
    ) -> Result<Self, PanelException> {
        let new_state = vec![vec![Pixel::default(); size.x]; size.y];
        assert_r!(
            top_left.x < bottom_right.x,
            PanelException::BadCoordinateException
        );
        assert_r!(
            top_left.y < bottom_right.y,
            PanelException::BadCoordinateException
        );
        Ok(Panel {
            previous_frame: new_state.clone(),
            next_frame: new_state.clone(),
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            state: PanelState::default(),
            writer: handle,
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
    pub fn run(&mut self) {
        loop {
            match self.command_receiver.try_recv() {
                Ok(cmd) => self.state.process_command(cmd),
                Err(_) => (),
            };

            if self.state.is_killed {
                break;
            }

            match self.frame_receiver.try_recv() {
                Ok(render_objects) => self.process_frame(render_objects),
                Err(_) => (),
            };
            // TODO: self.render_frame();
            // TODO: self.push_frame();
        }
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
    pub fn process_frame(&mut self, _render_objects: Vec<RenderObject>) {}

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
        size: Usize2d,
        top_left: Usize2d,
        bottom_right: Usize2d,
        frame_receiver: Receiver<Vec<RenderObject>>,
        command_receiver: Receiver<PanelCommandEnum>,
        handle: Box<dyn Handle>,
    ) -> Result<JoinHandle<()>, PanelException> {
        let mut w = Panel::init(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            handle,
        )?;
        let window_closure = move || {
            w.run();
        };
        Ok(spawn(window_closure))
    }
    /// Test if a coordinate is inside the panel
    ///
    /// # Arguments
    ///
    /// * `coord` - the coordinate to be tested
    ///
    /// # Returns
    /// A boolean value confirming wether the coordinate provided is inside the panel's coordinates
    ///
    /// # Example
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
    pub fn coord_is_in_panel(&mut self, coord: Usize2d) -> bool {
        coord.x >= self.top_left.x
            && coord.x <= self.bottom_right.x
            && coord.y >= self.top_left.y
            && coord.y <= self.bottom_right.y
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc::channel, thread::sleep, time::Duration};

    use crate::{
        panel::{command_enum::PanelCommandEnum, errors::PanelException},
        shared::{frame::Pixel, usize2d::Usize2d},
        writer::memory_handle::MemoryHandle,
    };

    use super::Panel;

    #[test]
    fn init() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let handle = Box::new(MemoryHandle::new());
        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let window = Panel::init(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            handle,
        );
        assert!(window.is_ok());
        let window = window.unwrap();

        assert_eq!(window.previous_frame, vec![vec![Pixel::default(); 10]; 20]);
        assert_eq!(window.next_frame, vec![vec![Pixel::default(); 10]; 20]);
    }

    #[test]
    fn init_fail_left_gt_right() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(1, 0);
        let bottom_right = Usize2d::new(0, 20);
        let handle = Box::new(MemoryHandle::new());
        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let window = Panel::init(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            handle,
        );

        assert!(window.is_err());
        let error = window.unwrap_err();
        assert!(error == PanelException::BadCoordinateException);
    }

    #[test]
    fn init_fail_top_gt_bottom() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(0, 1);
        let bottom_right = Usize2d::new(10, 0);
        let handle = Box::new(MemoryHandle::new());
        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let window = Panel::init(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            handle,
        );
        assert!(window.is_err());
        let error = window.unwrap_err();
        assert!(error == PanelException::BadCoordinateException);
    }

    #[test]
    fn run_and_kill() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let handle = Box::new(MemoryHandle::new());
        let (_, frame_receiver) = channel();
        let (command_sender, command_receiver) = channel();

        let handle = Panel::init_run_async(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            handle,
        );
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
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let handle = Box::new(MemoryHandle::new());
        let (_frame_sender, frame_receiver) = channel();
        let (command_sender, command_receiver) = channel();

        let handle = Panel::init_run_async(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            handle,
        );

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
    fn valid_coordinate() {
        let top_left = Usize2d::new(5, 7);
        let bottom_right = Usize2d::new(15, 20);

        let too_far_left = Usize2d::new(4, 8);
        let too_far_top = Usize2d::new(7, 6);
        let too_far_right = Usize2d::new(24, 8);
        let too_far_bottom = Usize2d::new(5, 26);

        let just_right = Usize2d::new(12, 16);

        let borderline_top = Usize2d::new(7, 7);
        let borderline_bottom = Usize2d::new(8, 20);
        let borderline_left = Usize2d::new(5, 10);
        let borderline_right = Usize2d::new(15, 10);

        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();
        let handle = Box::new(MemoryHandle::new());
        let w = Panel::init(
            Usize2d::default(),
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            handle,
        );
        assert!(w.is_ok());
        let mut w = w.unwrap();

        assert!(
            !w.coord_is_in_panel(too_far_left.clone()),
            "This coordinate ({}) is outside of the boundary of the panel (top_left: ({}), bottom_right: ({})",
            too_far_left,
            w.top_left,
            w.bottom_right
        );
        assert!(
            !w.coord_is_in_panel(too_far_top.clone()),
            "This coordinate ({}) is outside of the boundary of the panel (top_left: ({}), bottom_right: ({})",
            too_far_top,
            w.top_left,
            w.bottom_right
        );
        assert!(
            !w.coord_is_in_panel(too_far_bottom.clone()),
            "This coordinate ({}) is outside of the boundary of the panel (top_left: ({}), bottom_right: ({})",
            too_far_bottom,
            w.top_left,
            w.bottom_right
        );
        assert!(
            !w.coord_is_in_panel(too_far_right.clone()),
            "This coordinate ({}) is outside of the boundary of the panel (top_left: ({}), bottom_right: ({})",
            too_far_right,
            w.top_left,
            w.bottom_right
        );
        assert!(
            w.coord_is_in_panel(just_right.clone()),
            "This coordinate ({}) is inside of the boundary of the panel (top_left: ({}), bottom_right: ({})",
            just_right,
            w.top_left,
            w.bottom_right
        );
        assert!(
            w.coord_is_in_panel(borderline_top.clone()),
            "This coordinate ({}) is on the boundary of the panel (top_left: ({}), bottom_right: ({})",
            borderline_top,
            w.top_left,
            w.bottom_right
        );
        assert!(
            w.coord_is_in_panel(borderline_bottom.clone()),
            "This coordinate ({}) is on the boundary of the panel (top_left: ({}), bottom_right: ({})",
            borderline_bottom,
            w.top_left,
            w.bottom_right
        );
        assert!(
            w.coord_is_in_panel(borderline_left.clone()),
            "This coordinate ({}) is on the boundary of the panel (top_left: ({}), bottom_right: ({})",
            borderline_left,
            w.top_left,
            w.bottom_right
        );
        assert!(
            w.coord_is_in_panel(borderline_right.clone()),
            "This coordinate ({}) is on the boundary of the panel (top_left: ({}), bottom_right: ({})",
            borderline_right,
            w.top_left,
            w.bottom_right
        );
    }
}
