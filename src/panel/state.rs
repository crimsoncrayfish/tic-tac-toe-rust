use crate::shared::usize2d::Usize2d;

use super::command_enum::PanelCommandEnum;

#[derive(Debug)]
pub struct PanelState {
    pub is_paused: bool,
    pub is_killed: bool,
    pub is_cleared: bool,
    pub new_buffer_size: Usize2d,
}
impl Default for PanelState {
    fn default() -> Self {
        PanelState {
            is_paused: false,
            is_killed: false,
            is_cleared: false,
            new_buffer_size: Usize2d::default(),
        }
    }
}
impl PanelState {
    pub fn process_command(&mut self, command: PanelCommandEnum) {
        match command {
            PanelCommandEnum::KillProcess => self.is_killed = true,
            PanelCommandEnum::PauseProcess => self.is_paused = true,
            PanelCommandEnum::ClearBuffer => self.is_cleared = true,
            PanelCommandEnum::ResizeBuffer => self.is_cleared = true,
        }
    }
}
