use crate::shared::usize2d::Usize2d;

use super::command_enum::WindowCommandEnum;

#[derive(Debug)]
pub struct WindowState {
    pub is_paused: bool,
    pub is_killed: bool,
    pub is_cleared: bool,
    pub new_buffer_size: Usize2d,
}
impl Default for WindowState {
    fn default() -> Self {
        WindowState {
            is_paused: false,
            is_killed: false,
            is_cleared: false,
            new_buffer_size: Usize2d::default(),
        }
    }
}
impl WindowState {
    pub fn process_command(&mut self, command: WindowCommandEnum) {
        match command {
            WindowCommandEnum::KillProcess => self.is_killed = true,
            WindowCommandEnum::PauseProcess => self.is_paused = true,
            WindowCommandEnum::ClearBuffer => self.is_cleared = true,
            WindowCommandEnum::ResizeBuffer => self.is_cleared = true,
        }
    }
}
