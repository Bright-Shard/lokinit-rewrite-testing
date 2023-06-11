use crate::library;

use std::{
    ffi::{c_char, c_void},
};

library! {
    [LibWaylandClient <-> "wayland-client"];

    pub fn wl_display_connect(what: *const c_char) -> *mut c_void;
    pub fn wl_display_disconnect(what: *mut c_void);
}
