#[derive(Debug)]
pub enum HandleError {
    WriteFailed,
    SetCursorLocationFailed,
    SetForegroundFailed,
    SetBackgroundFailed,
    LockFailed,
}
