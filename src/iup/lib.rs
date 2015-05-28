//! Rust Binding for [IUP][1] -- A Portable User Interface Toolkit.
//!
//! [1]: http://www.tecgraf.puc-rio.br/iup/
//!
// TODO IMPROVE CRATE DOC ^

extern crate libc;
extern crate iup_sys;

use std::result::Result;
use std::ptr;

#[macro_use]
mod macros;

#[macro_use]
pub mod element;
pub use element::Handle;
pub use element::Element;

pub mod callback;

pub mod dialog;
pub mod layout;
pub mod control;


/// Initialises IUP toolkit, calls `f` for user initialization and runs the application.
///
/// All iup-rust functions, objects and methods must be used within the bounds of the `f` closure.
/// Such closure must  return a `Result` indicating whether the user initialization was successful.
///
/// This function will eturn only after the gui application is closed.
///
/// Returns `Ok` if the IUP initialization and initialization were successful. `Err` otherwise,
/// forwarding the user error if possible.
pub fn with_iup<F: FnOnce() -> Result<(), String>>(f: F) -> Result<(), String> {
    unsafe {
        match iup_sys::IupOpen(ptr::null(), ptr::null()) {
            // TODO improve those Errs, those strings don't look quite useful for matching!
            iup_sys::IUP_NOERROR => {},
            iup_sys::IUP_OPENED => return Err("IUP_OPENED: iup::open called while already open.".into()),
            iup_sys::IUP_ERROR => return Err("IUP_ERROR: X-Windows is not initialized".into()),
            _ => unreachable!(),
        };
        // TODO make f return a Result<ADialogType, String> and .show it ourselves instead since
        // IUP asks for at least one visible dialog to be show up during the IupMainLoop otherwise
        // the app will freeze. If to archive this one uses a specialized type for dialogs (just
        // a thought) consider changing the return type of functions like Element::dialog that
        // returns the handle to a dialog.
        let result = f();
        if result.is_ok() {
            // IupMainLoop always returns IUP_NOERROR.
            iup_sys::IupMainLoop();
        }
        iup_sys::IupClose();
        result
    }
}

/// Returns a string with the IUP version number.
pub fn version() -> String {
    string_from_c_str!(unsafe { iup_sys::IupVersion() })
}

/// Returns a number indicating the IUP version.
pub fn version_number() -> i32 {
    unsafe { iup_sys::IupVersionNumber() as i32 }
}
