#[derive(Debug, PartialEq, Eq)]
pub enum PanelError {
    BadCoordinate,
    OutOfBounds,
    BadRenderObject,
    WriteFailed,
    WriteLocationFailed,
    ReceiveDisconnect,
}
impl From<std::io::Error> for PanelError {
    fn from(_value: std::io::Error) -> Self {
        Self::WriteFailed
    }
}
