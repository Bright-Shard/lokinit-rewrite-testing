use super::dl::Library;
use std::{
    ffi::{c_char, c_void},
    mem::transmute,
    ptr,
};

pub struct WaylandClient {
    display: *mut c_void,
    disconnect_fn: extern "C" fn(*mut c_void),
}
impl WaylandClient {
    pub fn new() -> Result<Self, String> {
        let lib = Library::new("libwayland-client.so")?;

        let connect_fn: extern "C" fn(*const c_char) -> *mut c_void =
            unsafe { transmute(lib.get_sym("wl_display_connect")?) };

        let display = connect_fn(ptr::null() as _);
        let disconnect_fn = unsafe { transmute(lib.get_sym("wl_display_disconnect")?) };

        Ok(Self {
            display,
            disconnect_fn,
        })
    }
}
impl Drop for WaylandClient {
    fn drop(&mut self) {
        (self.disconnect_fn)(self.display);
        println!("Drop called, disconnected from Wayland");
    }
}
