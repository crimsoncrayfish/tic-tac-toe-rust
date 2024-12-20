use std::{
    sync::mpsc::Receiver,
    thread::{spawn, JoinHandle},
};

use crate::{
    assert_r,
    shared::{frame::Pixel, usize2d::Usize2d},
};

use super::{command_enum::WindowCommandEnum, errors::WindowException, state::WindowState};

#[derive(Debug)]
pub struct Window {
    previous_frame: Vec<Vec<Pixel>>,
    next_frame: Vec<Vec<Pixel>>,
    top_left: Usize2d,
    bottom_right: Usize2d,
    frame_receiver: Receiver<Vec<Vec<Pixel>>>,
    command_receiver: Receiver<WindowCommandEnum>,
    state: WindowState,
}
impl Window {
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
        frame_receiver: Receiver<Vec<Vec<Pixel>>>,
        command_receiver: Receiver<WindowCommandEnum>,
    ) -> Result<Self, WindowException> {
        let new_state = vec![vec![Pixel::default(); size.x]; size.y];
        assert_r!(
            top_left.x < bottom_right.x,
            WindowException::BadCoordinateException
        );
        assert_r!(
            top_left.y < bottom_right.y,
            WindowException::BadCoordinateException
        );
        Ok(Window {
            previous_frame: new_state.clone(),
            next_frame: new_state.clone(),
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
            state: WindowState::default(),
        })
    }

    /// Run the window process
    ///
    /// #Returns
    ///
    /// Does placeholder stuff
    ///
    /// # Examples
    ///
    /// ```
    /// let window= Window::init(size, top_left, bottom_right, frame_receiver, command_receiver);
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
        }
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
    /// #Returns
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
    /// let handle= Window::run_async(size, top_left, bottom_right, frame_receiver, command_receiver);
    /// handle.join().unwrap();
    /// ```
    pub fn init_run_async(
        size: Usize2d,
        top_left: Usize2d,
        bottom_right: Usize2d,
        frame_receiver: Receiver<Vec<Vec<Pixel>>>,
        command_receiver: Receiver<WindowCommandEnum>,
    ) -> Result<JoinHandle<()>, WindowException> {
        let mut w = Window::init(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
        )?;
        let window_closure = move || {
            w.run();
        };
        Ok(spawn(window_closure))
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc::channel, thread::sleep, time::Duration};

    use crate::{
        shared::{frame::Pixel, usize2d::Usize2d},
        windows::{command_enum::WindowCommandEnum, errors::WindowException},
    };

    use super::Window;

    #[test]
    fn init() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let window = Window::init(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
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
        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let window = Window::init(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
        );
        assert!(window.is_err());
        let error = window.unwrap_err();
        assert!(error == WindowException::BadCoordinateException);
    }

    #[test]
    fn init_fail_top_gt_bottom() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(0, 1);
        let bottom_right = Usize2d::new(10, 0);
        let (_, frame_receiver) = channel();
        let (_, command_receiver) = channel();

        let window = Window::init(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
        );
        assert!(window.is_err());
        let error = window.unwrap_err();
        assert!(error == WindowException::BadCoordinateException);
    }

    #[test]
    fn run_and_kill() {
        let size = Usize2d::new(10, 20);
        let top_left = Usize2d::new(0, 0);
        let bottom_right = Usize2d::new(10, 20);
        let (_, frame_receiver) = channel();
        let (command_sender, command_receiver) = channel();

        let handle = Window::init_run_async(
            size,
            top_left,
            bottom_right,
            frame_receiver,
            command_receiver,
        );
        let result = command_sender.send(WindowCommandEnum::KillProcess);
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
}
