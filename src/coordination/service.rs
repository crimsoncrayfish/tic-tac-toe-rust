use crate::{
    shared::{frame::Frame, usize2d::Usize2d},
    windows::window::Window,
};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct CoordinatorService {
    state: bool,
    windows: Vec<Window>,
}

impl CoordinatorService {
    pub fn init() -> Self {
        CoordinatorService {
            state: true,
            windows: Vec::new(),
        }
    }
    pub fn new_sender_receiver<T>() -> (Sender<T>, Receiver<T>) {
        let (sender, receiver): (Sender<T>, Receiver<T>) = channel();
        (sender, receiver)
    }

    pub fn new_window(frame_receiver: Receiver<Frame>, resize_receiver: Receiver<Usize2d>) {
        //-> Result<Window, WindowExeption>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let service = CoordinatorService::init();
        assert_eq!(service.state, true, "After initialization, the service should have a property called state that is set to 'true'");
        assert_eq!(service.windows.len(), 0, "After initialization, the service should have a property called windowa that is an empty Vec of Window");
    }
}
