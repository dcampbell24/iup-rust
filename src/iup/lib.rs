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
use std::mem;
use std::ptr;
use std::slice::SliceExt;
use libc::{c_char, c_int};

unsafe fn vec_to_c_array(v: Vec<Ihandle>) -> *mut *mut sys::Ihandle {
    let mut raw_v = Vec::with_capacity(v.len());
    for ih in v {
        raw_v.push(ih.ptr);
    }
    let null : *const sys::Ihandle = ptr::null();
    raw_v.push(mem::transmute(null));
    raw_v.as_mut_ptr()
}

#[allow(missing_copy_implementations)]
pub struct Ihandle {
    ptr: *mut sys::Ihandle,
}

impl Ihandle {
    fn from_ptr(ih: *mut sys::Ihandle) -> Ihandle {
        if ih.is_null() {
            panic!("Failed to create Ihandle.")
        } else {
            Ihandle { ptr: ih }
        }
    }
}

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

pub fn main_loop() {
    let ok = unsafe { sys::IupMainLoop() };
    assert_eq!(ok, sys::IUP_NOERROR);
}

pub fn show(ih: &mut Ihandle) -> IupResult {
    match unsafe { sys::IupShow(ih.ptr) } {
        sys::IUP_NOERROR => Ok(()),
        sys::IUP_ERROR => Err("IUP_ERROR: unknown error".to_string()),
        _ => unreachable!(),
    }
}

pub fn hbox(elements: Vec<Ihandle>) -> Ihandle {
    unsafe { Ihandle::from_ptr(sys::IupHboxv(vec_to_c_array(elements))) }
}

// Elements

pub fn label(text: &str) -> Ihandle {
    let text_c = CString::from_slice(text.as_bytes());
    unsafe { Ihandle::from_ptr(sys::IupLabel(text_c.as_ptr())) }
}

pub fn dialog(ih: Ihandle) -> Ihandle {
    unsafe { Ihandle::from_ptr(sys::IupDialog(ih.ptr)) }
}

pub fn button(text: &str) -> Ihandle {
    let text_c = CString::from_slice(text.as_bytes());
    let action: *const c_char = ptr::null();
    unsafe { Ihandle::from_ptr(sys::IupButton(text_c.as_ptr(), action)) }
}

pub fn text() -> Ihandle {
    let action: *const c_char = ptr::null();
    unsafe { Ihandle::from_ptr(sys::IupText(action)) }
}
