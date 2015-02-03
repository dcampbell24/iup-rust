//! Rust high level bindings for [IUP][1] -- A Portable User Interface Toolkit
//!
//! [1]: http://www.tecgraf.puc-rio.br/iup/
//!
//! The functions in this crate are a thin wrapper around the IUP functions unless noted
//! otherwise. Refer to the IUP website for their full documentation.
//!

extern crate libc;
extern crate "iup-sys" as sys;

use std::ffi::CString;
use std::ptr;
use sys::Ihandle;
use libc::{c_char, c_int};

pub type IupResult = Result<(), String>;

/// Initializes the IUP toolkit. Must be called before any other IUP function.
pub fn open() -> IupResult {
    let p1 : *const c_int = ptr::null();
    let p2 : *const *const *const c_char = ptr::null();

    match unsafe { sys::IupOpen(p1, p2) } {
        sys::IUP_NOERROR => Ok(()),
        sys::IUP_OPENED => Err("IUP_OPENED: iup::open called while already open.".to_string()),
        sys::IUP_ERROR => Err("IUP_ERROR: X-Windows is not initialized".to_string()),
        _ => unreachable!(),
    }
}

pub fn close() {
    unsafe { sys::IupClose(); }
}

pub fn label(s: &str) -> &mut Ihandle {
    let c_string = CString::from_slice(s.as_bytes());
    unsafe { &mut *(sys::IupLabel(c_string.as_ptr())) } 
}

pub fn main_loop() {
    let ok = unsafe { sys::IupMainLoop() };
    assert_eq!(ok, sys::IUP_NOERROR);
}

pub fn dialog(ih: &mut Ihandle) -> &mut Ihandle {
    unsafe { &mut *(sys::IupDialog(ih)) }
}

pub fn show(ih: &mut Ihandle) -> IupResult {
    match unsafe { sys::IupShow(ih) } {
        sys::IUP_NOERROR => Ok(()),
        sys::IUP_ERROR => Err("IUP_ERROR: unknown error".to_string()),
        _ => unreachable!(),
    }
}
