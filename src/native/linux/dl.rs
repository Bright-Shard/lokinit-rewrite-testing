mod ffi {
    use core::ffi::{c_char, c_int, c_void};

    // https://github.com/nagisa/rust_libloading/blob/master/src/os/unix/consts.rs
    pub const RTLD_LAZY: c_int = 1;
    pub const RTLD_NOW: c_int = 2;

    // Link to the dynamic linker & its functions
    // Docs: https://linux.die.net/man/3/dlopen
    #[link(name = "dl")]
    extern "C" {
        pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
        pub fn dlerror() -> *const c_char;
        pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        pub fn dlclose(handle: *mut c_void) -> c_int;
    }
}

use std::ffi::{c_void, CStr, CString};

/// A safe wrapper around dlerror
fn get_dlerror() -> Option<String> {
    let err = unsafe { ffi::dlerror() };
    if err.is_null() {
        None
    } else {
        let c_str = unsafe { CStr::from_ptr(err) };
        Some(c_str.to_str().unwrap().to_owned())
    }
}

/// A wrapper for all the dl methods
pub struct Library {
    handle: *mut c_void,
}
impl Library {
    /// Create a new Library from the name of its file.
    ///
    /// Usually, Linux libraries are in the form 'lib<name>.so'. This will *not*
    /// add the 'lib' or '.so' for you; you're responsible for that.
    pub fn new(name: &str) -> Result<Self, String> {
        let c_str = CString::new(name).unwrap();
        let lib = unsafe { ffi::dlopen(c_str.as_ptr(), ffi::RTLD_NOW) };

        if lib.is_null() {
            Err(get_dlerror().unwrap())
        } else {
            Ok(Self { handle: lib })
        }
    }

    pub fn get_sym(&self, sym: &str) -> Result<*mut c_void, String> {
        let c_str = CString::new(sym).unwrap();
        let sym = unsafe { ffi::dlsym(self.handle, c_str.as_ptr()) };

        if sym.is_null() {
            Err(get_dlerror().unwrap())
        } else {
            Ok(sym)
        }
    }
}
