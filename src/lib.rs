//! # IUP
//!
//! [IUP][1] is a multi-platform toolkit for building graphical user interfaces.
//!
//! IUP's purpose is to allow a program to run in different systems without changes - the toolkit
//! provides the application portability. Supported systems include: GTK+, Motif and Windows. 
//!
//! IUP has some advantages over other interface toolkits available:
//!
//! + **Simplicity:** due to the small number of functions and to its attribute mechanism,
//!   the learning curve for a new user is often faster.
//! + **Portability:** the same functions are implemented in each one of the platforms, thus
//!   assuring the interface system's portability.
//! + **Customization:** the dialog specification language (LED) is a mechanisms in which it
//!   is possible to customize an application for a specific user with a simple-syntax text file.
//! + **Flexibility:** its abstract layout mechanism provides flexibility to dialog creation.
//! + **Extensibility:** the programmer can create new interface elements as needed.
//!
//! # The Rust Binding
//!
//! The Rust binding provides a way to do things in a more Rustic way but without moving out of
//! IUP base nameclatures and philosophy in such a way that one can program on this binding by reading the
//! original [IUP documentation][1].
//!
//! Everything created by IUP is so called a [`element`](element/) which imeplements the `Element`
//! trait. Each of those objects can also be encapsulated in a `Handle` element which contains
//! the common functionalities and allow downcasting back to the original element type.
//!
//! The library is divided in a few submodules in a way that it matches the IUP documentation
//! division of elements:
//!  
//!   + The [controls](control/) submodule contains the user interface controls.
//!   + The [layout](layout/) submodule contains the abstract layout composition controls.
//!   + The [dialogs](dialog/) submodule contains the dialog definitions, such as windows,
//!     message boxes, file selection, color selection between others.
//!
//! Each of those elements communicates with the programmer by the means of [callbacks](callback/)
//! and attributes. Callbacks are closures that gets called when *something* happens with the
//! control such as a  button click and attributes are the way to set and get specific properties
//! of the element such as it's design or value.
//!
//! Currently attributes are not individual methods specific to each element but that may be a
//! thing in the future.
//!
//! The binding is built in a way one can build controls or even the entire window of the
//! application in a single expression in a very expressive way, for example:
//!
//! ```ignore
//! Dialog::new(
//!     Radio::new(
//!         VBox::new(elements![
//!             Toggle::with_title("Option 1")
//!                     .set_attrib("TIP", "I am a tip!")
//!                     .set_attrib("VALUE", "ON")
//!                     .set_action(|(_, state)| println!("Option 1 = {}", state)),
//!             Toggle::with_title("Option 2")
//!                     .set_action(|(_, state)| println!("Option 2 = {}", state))
//!                     .set_valuechanged_cb(|_| println!("Option 2 changed!!!")),
//!         ])
//!     )
//! ).set_attrib("TITLE", "Hello IUP!")
//!  .show();
//! ```
//!
//! This is just a example of one of the many ways one could build the GUI creation code,
//! this model opens a lot of possibilities on this matter. There's also the possibility to use
//! the [LED](led/) file format and allow users to easily modify the user interface with no
//! programming experience.
//!
//! [1]: http://www.tecgraf.puc-rio.br/iup/
//!

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

#[macro_use]
pub mod callback;

pub mod dialog;
pub mod layout;
pub mod control;

pub mod led;
pub mod image;

pub mod prelude;

pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Orientation {
    #[doc(hidden)]
    pub fn as_cstr(self) -> *const libc::c_char {
        use self::Orientation::*;
        match self {
            Vertical => cstr!("VERTICAL"),
            Horizontal => cstr!("HORIZONTAL"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitError {
    AlreadyOpen,
    Error,
}

/// Initialises IUP toolkit, calls `f` for user initialization and runs the application.
///
/// All iup-rust functions, objects and methods must be used within the bounds of the `f` closure.
/// Such closure must  return a `Result` indicating whether the user initialization was successful.
///
/// This function will eturn only after the gui application is closed.
///
/// Returns `Ok` if the IUP initialization and user initialization were successful. `Err` otherwise.
pub fn with_iup<T, E, F: FnOnce() -> Result<T, E>>(f: F) -> Result<(), InitError> {

    match unsafe { iup_sys::IupOpen(ptr::null(), ptr::null()) } {
        iup_sys::IUP_NOERROR => {},
        iup_sys::IUP_OPENED => return Err(InitError::AlreadyOpen),
        iup_sys::IUP_ERROR => return Err(InitError::Error),
        _ => unreachable!(),
    };

    // Turn UTF-8 mode ON since Rust uses UTF-8 on strings.
    match element::global("DRIVER").unwrap().as_ref() {
        "GTK" | "Win32" => unsafe {
            iup_sys::IupSetGlobal(cstr!("UTF8MODE"), cstr!("YES"));
            iup_sys::IupSetGlobal(cstr!("UTF8MODE_FILE"), cstr!("YES"));
        },
        _ => println!("Warning: This IUP driver does not seem to support UTF-8!"),
    }

    if f().is_ok() {
        // IupMainLoop always returns IUP_NOERROR.
        unsafe { iup_sys::IupMainLoop(); }
    }
    unsafe { iup_sys::IupClose(); }
    Ok(())
}

/// Returns a string with the IUP version number.
pub fn version() -> String {
    string_from_cstr!(unsafe { iup_sys::IupVersion() })
}

/// Returns a number indicating the IUP version.
pub fn version_number() -> i32 {
    unsafe { iup_sys::IupVersionNumber() as i32 }
}
