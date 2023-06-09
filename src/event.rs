use crate::window::Window;

#[derive(Debug, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum ClickStatus {
    /// The mouse/finger just started clicking/pressing
    Started,
    /// The mouse/finger moved on the screen
    Moved(u32, u32),
    /// The mouse/finger just stopped clicking/pressing
    Stopped,
}

pub enum WindowEvent {
    /// The OS requested a redraw of the window
    Redraw,
}

/// Events happening in a window
pub enum Event {
    /// The OS requested a redraw of this window
    Redraw,
    /// The window should update (but not necessarily redraw its view)
    Update,
    /// The user clicked the window
    Click(ClickStatus, MouseButton),
    /// The user touched the window
    Touch(u8, ClickStatus),
}
