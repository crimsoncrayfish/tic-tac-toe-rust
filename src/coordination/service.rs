use crate::windows::window::Window;

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
