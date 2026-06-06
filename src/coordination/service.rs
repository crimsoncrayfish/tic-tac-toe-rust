use crate::{
    panel::actual::Panel,
    shared::{pixel_grid::Frame, usize2d::Usize2d},
};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct CoordinatorService {
    _state: bool,
    _panels: Vec<Panel>,
}

impl CoordinatorService {
    pub fn init() -> Self {
        CoordinatorService {
            _state: true,
            _panels: Vec::new(),
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
        assert!(service._state, "After initialization, the service should have a property called state that is set to 'true'");
        assert_eq!(service._panels.len(), 0, "After initialization, the service should have a property called panels that is an empty Vec of Window");
    }
}
