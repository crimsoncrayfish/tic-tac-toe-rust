use crate::{
    panel::panel::Panel,
    shared::{frame::Frame, usize2d::Usize2d},
};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct CoordinatorService {
    state: bool,
    panels: Vec<Panel>,
}

impl CoordinatorService {
    pub fn init() -> Self {
        CoordinatorService {
            state: true,
            panels: Vec::new(),
        }
    }
    pub fn new_sender_receiver<T>() -> (Sender<T>, Receiver<T>) {
        let (sender, receiver): (Sender<T>, Receiver<T>) = channel();
        (sender, receiver)
    }

    pub fn new_window(_frame_receiver: Receiver<Frame>, _resize_receiver: Receiver<Usize2d>) {
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
        assert_eq!(service.panels.len(), 0, "After initialization, the service should have a property called windowa that is an empty Vec of Window");
    }
}
