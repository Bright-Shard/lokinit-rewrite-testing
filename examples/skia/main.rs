use lokinit::prelude::*;

pub struct MainWindow;

impl Window for MainWindow {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Redraw => {}
            Event::Update => {}
            Event::Click(status, button) => {}
            Event::Touch(finger, status) => {}
        }
    }
}

fn main() {
    App::new().add_window(MainWindow).start();
}
