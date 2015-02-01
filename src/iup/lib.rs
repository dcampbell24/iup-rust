extern crate libc;
extern crate "iup-sys" as sys;

use std::ffi::CString;
use std::ptr;
use sys::Ihandle;
use libc::{c_char, c_int};

// FIXME: Properly pass argc and argv to IupOpen.
pub fn open() -> bool {
    let p1 : *const c_int = ptr::null();
    let p2 : *const *const *const c_char = ptr::null();
    let ok = unsafe { sys::IupOpen(p1, p2) };
    ok == sys::IUP_NOERROR
}

pub fn label(s: &str) -> &mut Ihandle {
    let c_string = CString::from_slice(s.as_bytes());
    unsafe { &mut *(sys::IupLabel(c_string.as_ptr())) } 
}

pub fn close() {
    unsafe { sys::IupClose(); }
}

pub fn main_loop() -> bool {
    unsafe { sys::IupMainLoop() == sys::IUP_NOERROR }
}

pub fn dialog(ih: &mut Ihandle) -> &mut Ihandle {
    unsafe { &mut *(sys::IupDialog(ih)) }
}

pub fn show(ih: &mut Ihandle) -> bool {
    let ok = unsafe { sys::IupShow(ih) };
    ok == sys::IUP_NOERROR
}
