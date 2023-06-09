use crate::event::Event;

pub struct WindowConfig {}

pub trait Window {
    fn handle_event(&mut self, event: Event);
}
