//! Event-driven communication.

use iup_sys;
use libc::{c_char, c_int};
use std::path::PathBuf;

#[macro_use]
mod macros;
pub mod callbacks;
pub use self::callbacks::*;

pub mod button; // TODO move to somewhere else?

/// Return this from a callback to tell the framework a non-default action to be performed.
///
/// Not all callbacks accepts `Close`, `Ignore` or `Continue`, check their respective docs.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CallbackReturn {
    /// The default `CallbackReturn`, does nothing when returned.
    Default,
    /// If this is returned from a callback, then when the callback returns the dialog containing
    /// the element on which the callback was invoked will be closed.
    Close,
    /// Callback specific, check the callback documentation to see if it accepts this return value
    /// and it's effect.
    Ignore,
    /// Callback specific, check the callback documentation to see if it accepts this return value
    /// and it's effect.
    Continue,
}

impl CallbackReturn {
    fn to_cb_return(self) -> iup_sys::CallbackReturn {
        use self::CallbackReturn::*;
        match self {
            Close => iup_sys::CallbackReturn::Close,
            Default => iup_sys::CallbackReturn::Default,
            Ignore => iup_sys::CallbackReturn::Ignore,
            Continue => iup_sys::CallbackReturn::Continue,
        }
    }
}

// This allows returning '()' from a callback instead of CallbackReturn.
impl From<()> for CallbackReturn {
    fn from(_: ()) -> CallbackReturn {
        CallbackReturn::Default
    }
}

pub trait Callback<Args> : 'static {
    fn on_callback(&mut self, args: Args) -> iup_sys::CallbackReturn; 
}

// TODO `Callback<Args> for F where F: FnMut<Args, Output=Out>`
// error: angle-bracket notation is not stable when used with the `Fn` family of traits,
//       use parentheses [E0215]
//
// This would allow the functions to receive a variadic number of arguments instead of a single
// tuple argument with a variadic length.

impl<Args, Out: Into<CallbackReturn>, F: 'static> Callback<Args> for F where F: FnMut(Args) -> Out {
    /// Because of the `impl From<()> for CallbackReturn`, closures that return `()` can be
    /// accepted by this impl.
    fn on_callback(&mut self, args: Args) -> iup_sys::CallbackReturn {
        let r = self(args).into();
        r.to_cb_return()
    }
}

/// This is a internal trait used to convert IUP C types into IUP Rust types in callbacks.
///
/// For instance BUTTON_CB has a `char* status` parameter that must be abstracted into another
/// type (e.g. `KeyStatus`).
///
/// This trait method `into_rust` is called from the `impl_callback!` macro.
#[doc(hidden)]
trait IntoRust<T> {
    fn into_rust(self) -> T;
}

impl IntoRust<i32> for c_int {
    fn into_rust(self) -> i32 {
        self as i32
    }
}

impl IntoRust<usize> for c_int {
    fn into_rust(self) -> usize {
        self as usize
    }
}

impl IntoRust<bool> for c_int {
    fn into_rust(self) -> bool {
        self != 0
    }
}

impl IntoRust<PathBuf> for *const c_char {
    fn into_rust(self) -> PathBuf {
        PathBuf::from(string_from_cstr!(self))
    }
}


// This is called right when a IUP element is being destroyed and it should free up all data
// associated with callbacks. Just use the `drop_callback!` macro for each callback implemented.
#[doc(hidden)]
pub unsafe fn drop_callbacks(ih: *mut iup_sys::Ihandle) {
    drop_callback!(ih, "ACTION");
    drop_callback!(ih, "LDESTROY_CB");
}
