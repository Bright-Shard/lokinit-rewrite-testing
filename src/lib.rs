mod app;
pub mod event;
pub mod keycode;
mod native;
mod window;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::event::{Event, MouseButton};
    pub use crate::window::Window;
}
