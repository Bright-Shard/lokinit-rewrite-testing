#[cfg(feature = "wayland")]
mod wayland;

#[cfg(feature = "x11")]
mod x11;

mod dl;

impl crate::application::Application {
    pub fn start(self) {
        #[cfg(feature = "wayland")]
        wayland::WaylandClient::new().unwrap();
    }
}
