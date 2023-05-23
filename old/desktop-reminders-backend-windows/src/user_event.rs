#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BackendUserEvent {
    /// A request has been made to repaint the window.
    RequestRepaint,
    /// The desktop has been brought to the foreground.
    DesktopShown,
    /// The desktop has been sent to the background.
    DesktopHidden,
}
