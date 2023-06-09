use crate::window::Window;

#[derive(Default)]
pub struct App {
    windows: Vec<Box<dyn Window>>,
}
impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_window(mut self, window: impl Window + 'static) -> Self {
        self.windows.push(Box::new(window));
        self
    }
}
