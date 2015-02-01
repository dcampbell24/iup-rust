extern crate libc;

use libc::{c_char, c_int};

pub const IUP_NOERROR: c_int = 0;

pub enum Ihandle {}

extern {
    pub fn IupOpen(argc: *const c_int, argv: *const *const *const c_char) -> c_int;
    pub fn IupShow(ih: *mut Ihandle) -> c_int;
    pub fn IupMainLoop() -> c_int;
    pub fn IupClose();
    pub fn IupLabel(title: *const c_char) -> *mut Ihandle;
    pub fn IupDialog(ih: *mut Ihandle) -> *mut Ihandle;
}
