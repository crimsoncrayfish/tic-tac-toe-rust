use std::{fmt::Debug, io::Write};

use crate::{rendering::colors::TerminalColors, shared::usize2d::Usize2d};

use super::handle_error::HandleError;

pub trait Handle: Write + Send + Debug {
    /// Set the locaction of the cursor on the buffer
    ///
    /// # Arguments
    ///
    /// * `coord` - the new location of the cursor as a `Usize2d`
    ///
    /// # Returns
    /// The result object indicating if the set was successfull with any error that could have
    /// occurred
    ///
    /// # Example
    ///
    /// ```
    /// my_handle.set_cursor_location(Usize2d::default());
    /// ```
    fn set_cursor_location(&mut self, coord: Usize2d) -> Result<(), HandleError>;
    /// Set the foreground color at the current coordinate on the buffer for this handle
    ///
    /// # Arguments
    ///
    /// * `color` - The new color of the foreground for the cell
    ///
    /// # Returns
    /// The result object indicating if the set was successfull with any error that could have
    /// occurred
    ///
    /// # Example
    ///
    /// ```
    /// my_handle.set_foreground_color(TerminalColors::Red);
    /// ```
    fn set_foreground_color(&mut self, color: TerminalColors) -> Result<(), HandleError>;
    /// Set the background color at the current coordinate on the buffer for this handle
    ///
    /// # Arguments
    ///
    /// * `color` - The new color of the background for the cell
    ///
    /// # Returns
    /// The result object indicating if the set was successfull with any error that could have
    /// occurred
    ///
    /// # Example
    ///
    /// ```
    /// my_handle.set_background_color(TerminalColors::Blue);
    /// ```
    fn set_background_color(&mut self, color: TerminalColors) -> Result<(), HandleError>;
}
