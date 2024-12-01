use std::time::Duration;

use crate::coordinate::Coord;
pub struct ConwaysSettings {
    pub x_len: usize,
    pub y_len: usize,
    pub cell_view_width: u16,
    pub cell_view_height: u16,
    pub round_duration: Duration,
    pub origin: Coord,
}
impl ConwaysSettings {
    pub fn init(x_len: usize, y_len: usize, duration: Duration) -> Self {
        ConwaysSettings {
            x_len,
            y_len,
            cell_view_width: 3,
            cell_view_height: 2,
            round_duration: duration,
            origin: Coord { x: 0, y: 0 },
        }
    }
}
