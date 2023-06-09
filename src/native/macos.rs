/// FFI functions in Swift, for Rust to use
mod swift {
    use std::ffi::c_void;
    use std::ptr;

    /// A pointer to Swift's app instance
    pub static mut APP: *mut c_void = ptr::null_mut();

    extern "C" {
        pub fn new_app() -> *mut c_void;
    }
}

/// FFI functions in Rust, for Swift to use
mod rust {
    use crate::app::App;

    /// Rust's app instance
    pub static mut APP: Option<App> = None;
}

use crate::app::App;

impl App {
    pub fn start(self) {
        unsafe {
            rust::APP = Some(self);
            swift::APP = swift::new_app();
        }
    }
}
