mod app;
mod event;
mod native;
mod window;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::event::{ClickStatus, Event, MouseButton};
    pub use crate::window::Window;
}
