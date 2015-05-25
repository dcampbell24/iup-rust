//! Rust high level bindings for [IUP][1] -- A Portable User Interface Toolkit
//!
//! [1]: http://www.tecgraf.puc-rio.br/iup/
//!
//! The functions in this crate are a thin wrapper around the IUP functions unless noted
//! otherwise. Refer to the IUP website for their full documentation.
//!

extern crate libc;
extern crate iup_sys;

mod macros;
pub mod callback;

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use libc::{c_char};
use std::result;
use std::path::Path;

pub use iup_sys::CallbackReturn;
pub use iup_sys::Ihandle as IhandleRaw;

pub type Result<T> = result::Result<T, String>; // TODO make the error a enum instead of a string!

fn slice_to_vvec(slice: &[Ihandle]) -> Vec<*mut iup_sys::Ihandle> {
    let mut raw_v: Vec<_> = slice.iter().map(|ih| ih.ptr).collect();
    raw_v.push(ptr::null_mut());
    raw_v
}

#[allow(missing_copy_implementations)]
pub struct Ihandle {
    ptr: *mut iup_sys::Ihandle,
}

pub struct Iup;

impl Ihandle {
    pub fn from_ptr(ih: *mut iup_sys::Ihandle) -> Ihandle {
        if ih.is_null() {
            panic!("Failed to create Ihandle.")
        } else {
            unsafe { iup_sys::IupSetCallback(ih, str_to_c_str!("LDESTROY_CB"), Ihandle::on_destroy) };
            Ihandle { ptr: ih }
        }
    }

    extern fn on_destroy(ih: *mut iup_sys::Ihandle) -> CallbackReturn {
        callback::on_destroy(ih);
        CallbackReturn::Default
    }
}

impl Clone for Ihandle {
    fn clone(&self) -> Ihandle {
        Ihandle { ptr: self.ptr }
    }
}


/// Associate a rust value with a string for later use in an IUP callback by
/// calling `get_rust_handle` with the same string.
pub unsafe fn set_rust_handle<T>(name: &str, rh: &mut T) -> Option<*mut IhandleRaw> {
    let ptr: *mut IhandleRaw = mem::transmute(rh);
    let name_c = CString::new(name).unwrap();
    let ih_old = iup_sys::IupSetHandle(name_c.as_ptr(), ptr);
    if ih_old.is_null() {
        None
    } else {
        Some(ih_old)
    }
}

/// Retrieve a rust value that was associated with a string using
/// `set_rust_handle`. This function is only intended for use in IUP callbacks.
pub unsafe fn get_rust_handle(name: &str) -> Option<*mut IhandleRaw> {
    let name_c = CString::new(name).unwrap();
    let ih = iup_sys::IupGetHandle(name_c.as_ptr());
    if ih.is_null() {
        None
    } else {
        Some(ih)
    }
}

/************************************************************************/
/*                        Main API                                      */
/************************************************************************/

pub static mut IS_IUP_ALIVE: bool = false;

pub fn open() -> Result<Iup> {
    match unsafe { iup_sys::IupOpen(ptr::null(), ptr::null()) } {
        iup_sys::IUP_NOERROR => {
            unsafe { assert!(IS_IUP_ALIVE == false); }
            unsafe { IS_IUP_ALIVE = true; }
            Ok(Iup)
        },
        iup_sys::IUP_OPENED => Err("IUP_OPENED: iup::open called while already open.".to_string()),
        iup_sys::IUP_ERROR => Err("IUP_ERROR: X-Windows is not initialized".to_string()),
        _ => unreachable!(),
    }
}

impl Drop for Iup
{
    fn drop(&mut self) {
        unsafe {
            assert!(IS_IUP_ALIVE == true);
            IS_IUP_ALIVE = false;
            iup_sys::IupClose();
        }
    }
}

// pub fn IupImageLibOpen();

pub fn main_loop() {
    let ok = unsafe { iup_sys::IupMainLoop() };
    assert_eq!(ok, iup_sys::IUP_NOERROR);
}

// pub fn IupLoopStep() -> c_int;
// pub fn IupLoopStepWait() -> c_int;
// pub fn IupMainLoopLevel() -> c_int;
// pub fn IupFlush();
// pub fn IupExitLoop();

// pub fn IupRecordInput(filename: *const c_char, mode: c_int) -> c_int;
// pub fn IupPlayInput(filename: *const c_char) -> c_int;

// pub fn IupUpdate(ih: *mut Ihandle);
// pub fn IupUpdateChildren(ih: *mut Ihandle);
// pub fn IupRedraw(ih: *mut Ihandle, children: c_int);
// pub fn IupRefresh(ih: *mut Ihandle);
// pub fn IupRefreshChildren(ih: *mut Ihandle);

// pub fn IupHelp(url: *const c_char) -> c_int;
// pub fn IupLoad(filename: *const c_char) -> *mut c_char;
// pub fn IupLoadBuffer(buffer: *const c_char) -> *mut c_char;

// pub fn IupVersion() -> *mut c_char;
// pub fn IupVersionDate() -> *mut c_char;
// pub fn IupVersionNumber() -> c_int;

// pub fn IupSetLanguage(lng: *const c_char);
// pub fn IupGetLanguage() -> *mut c_char;
// pub fn IupSetLanguageString(name: *const c_char, str: *const c_char);
// pub fn IupStoreLanguageString(name: *const c_char, str: *const c_char);
// pub fn IupGetLanguageString(name: *const c_char) -> *mut c_char;
// pub fn IupSetLanguagePack(ih: *mut Ihandle);

pub fn destroy(ih: Ihandle) {
    unsafe { iup_sys::IupDestroy(ih.ptr); }
}

// pub fn IupDetach(child: *mut Ihandle);
// pub fn IupAppend(ih: *mut Ihandle, child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupInsert(ih: *mut Ihandle, ref_child: *mut Ihandle, child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupGetChild(ih: *mut Ihandle, pos: c_int) -> *mut Ihandle;
// pub fn IupGetChildPos(ih: *mut Ihandle, child: *mut Ihandle) -> c_int;
// pub fn IupGetChildCount(ih: *mut Ihandle) -> c_int;
// pub fn IupGetNextChild(ih: *mut Ihandle, child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupGetBrother(ih: *mut Ihandle) -> *mut Ihandle;
// pub fn IupGetParent(ih: *mut Ihandle) -> *mut Ihandle;
// pub fn IupGetDialog(ih: *mut Ihandle) -> *mut Ihandle;
// pub fn IupGetDialogChild(ih: *mut Ihandle, name: *const c_char) -> *mut Ihandle;
// pub fn IupReparent(ih: *mut Ihandle, new_parent: *mut Ihandle, ref_child: *mut Ihandle) -> c_int;

// pub fn IupPopup(ih: *mut Ihandle, x: c_int, y: c_int) -> c_int;

pub fn show(ih: &mut Ihandle) -> Result<()> {
    match unsafe { iup_sys::IupShow(ih.ptr) } {
        iup_sys::IUP_NOERROR => Ok(()),
        iup_sys::IUP_ERROR => Err("IUP_ERROR: unknown error".to_string()),
        _ => unreachable!(),
    }
}

// pub fn IupShowXY(ih: *mut Ihandle, x: c_int, y: c_int) -> c_int;
// pub fn IupHide(ih: *mut Ihandle) -> c_int;
// pub fn IupMap(ih: *mut Ihandle) -> c_int;
// pub fn IupUnmap(ih: *mut Ihandle);

// pub fn IupResetAttribute(ih: *mut Ihandle, name: *const c_char);
// pub fn IupGetAllAttributes(ih: *mut Ihandle, names: *mut *mut c_char, n: c_int) -> c_int;
// pub fn IupSetAtt(handle_name: *const c_char, ih: *mut Ihandle, name: *const c_char, ...) -> *mut Ihandle;
// pub fn IupSetAttributes(ih: *mut Ihandle, str: *const c_char) -> *mut Ihandle;
// pub fn IupGetAttributes(ih: *mut Ihandle) -> *mut c_char;

// pub fn IupSetAttribute(ih: *mut Ihandle, name: *const c_char, value: *const c_char);

pub fn set_str_attribute(ih: &mut Ihandle, name: &str, value: &str) {
    let name_c = CString::new(name).unwrap();
    let value_c = CString::new(value).unwrap();
    unsafe { iup_sys::IupSetStrAttribute(ih.ptr, name_c.as_ptr(), value_c.as_ptr()); }
}

// pub fn IupSetStrf(ih: *mut Ihandle, name: *const c_char, format: *const c_char, ...);
// pub fn IupSetInt(ih: *mut Ihandle, name: *const c_char, value: c_int);
// pub fn IupSetFloat(ih: *mut Ihandle, name: *const c_char, value: c_float);
// pub fn IupSetDouble(ih: *mut Ihandle, name: *const c_char, value: c_double);
// pub fn IupSetRGB(ih: *mut Ihandle, name: *const c_char, r: c_uchar, g: c_uchar, b: c_uchar);

pub fn get_attribute(ih: &mut Ihandle, name: &str) -> Option<String> {
    let name_c = CString::new(name).unwrap();
    let value_c = unsafe { iup_sys::IupGetAttribute(ih.ptr, name_c.as_ptr()) };
    if value_c.is_null() {
        None
    } else {
        let buf = unsafe { CStr::from_ptr(value_c).to_bytes() };
        match String::from_utf8(buf.to_vec()) {
            Ok(s) => Some(s),
            Err(_) => None
        }
    }
}

// pub fn IupGetInt(ih: *mut Ihandle, name: *const c_char) -> c_int;
// pub fn IupGetInt2(ih: *mut Ihandle, name: *const c_char) -> c_int;
// pub fn IupGetIntInt(ih: *mut Ihandle, name: *const c_char, i1: *mut c_int, i2: *mut c_int) -> c_int;
// pub fn IupGetFloat(ih: *mut Ihandle, name: *const c_char) -> c_float;
// pub fn IupGetDouble(ih: *mut Ihandle, name: *const c_char) -> c_double;
// pub fn IupGetRGB(ih: *mut Ihandle, name: *const c_char, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar);
// pub fn IupSetAttributeId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: *const c_char);
// pub fn IupSetStrAttributeId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: *const c_char);
// pub fn IupSetStrfId(ih: *mut Ihandle, name: *const c_char, id: c_int, format: *const c_char, ...);
// pub fn IupSetIntId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: c_int);
// pub fn IupSetFloatId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: c_float);
// pub fn IupSetDoubleId(ih: *mut Ihandle, name: *const c_char, id: c_int, value: c_double);
// pub fn IupSetRGBId(ih: *mut Ihandle, name: *const c_char, id: c_int, r: c_uchar, g: c_uchar, b: c_uchar);

// pub fn IupGetAttributeId(ih: *mut Ihandle, name: *const c_char, id: c_int) -> *mut c_char;
// pub fn IupGetIntId(ih: *mut Ihandle, name: *const c_char, id: c_int) -> c_int;
// pub fn IupGetFloatId(ih: *mut Ihandle, name: *const c_char, id: c_int) -> c_float;
// pub fn IupGetDoubleId(ih: *mut Ihandle, name: *const c_char, id: c_int) -> c_double;
// pub fn IupGetRGBId(ih: *mut Ihandle, name: *const c_char, id: c_int, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar);

// pub fn IupSetAttributeId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: *const c_char);
// pub fn IupSetStrAttributeId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: *const c_char);
// pub fn IupSetStrfId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, format: *const c_char, ...);
// pub fn IupSetIntId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: c_int);
// pub fn IupSetFloatId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: c_float);
// pub fn IupSetDoubleId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, value: c_double);
// pub fn IupSetRGBId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, r: c_uchar, g: c_uchar, b: c_uchar);

// pub fn IupGetAttributeId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int) -> *mut c_char;
// pub fn IupGetIntId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int) -> c_int;
// pub fn IupGetFloatId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int) -> c_float;
// pub fn IupGetDoubleId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int) -> c_double;
// pub fn IupGetRGBId2(ih: *mut Ihandle, name: *const c_char, lin: c_int, col: c_int, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar);

// pub fn IupSetGlobal(name: *const c_char, value: *const c_char);
// pub fn IupSetStrGlobal(name: *const c_char, value: *const c_char);
// pub fn IupGetGlobal(name: *const c_char) -> *mut c_char;

// pub fn IupSetFocus(ih: *mut Ihandle) -> *mut Ihandle;
// pub fn IupGetFocus() -> *mut Ihandle;
// pub fn IupPreviousField(ih: *mut Ihandle) -> *mut Ihandle;
// pub fn IupNextField(ih: *mut Ihandle) -> *mut Ihandle;

// pub fn IupGetCallback(ih: *mut Ihandle, name: *const c_char) -> Icallback;

pub fn set_callback(ih: &mut Ihandle, name: &str, callback: iup_sys::Icallback) -> iup_sys::Icallback {
    let name_c = CString::new(name).unwrap();
    unsafe { iup_sys::IupSetCallback(ih.ptr, name_c.as_ptr(), callback) }
}

// pub fn IupSetCallbacks(ih: *mut Ihandle, name: *const c_char, func: Icallback, ...) -> *mut Ihandle;

// pub fn IupGetFunction(name: *const c_char) -> Icallback;
// pub fn IupSetFunction(name: *const c_char, func: Icallback) -> Icallback;

pub fn get_handle(name: &str) -> Option<Ihandle> {
    let name_c = CString::new(name).unwrap();
    let ih = unsafe { iup_sys::IupGetHandle(name_c.as_ptr()) };
    if ih.is_null() {
        None
    } else {
        Some(Ihandle { ptr: ih })
    }
}

pub fn set_handle(name: &str, ih: &mut Ihandle) -> Option<Ihandle> {
    let name_c = CString::new(name).unwrap();
    let ih_old = unsafe { iup_sys::IupSetHandle(name_c.as_ptr(), ih.ptr) };
    if ih_old.is_null() {
        None
    } else {
        Some(Ihandle { ptr: ih_old })
    }
}

// pub fn IupGetAllNames(names: *mut *mut c_char, n: c_int) -> c_int;
// pub fn IupGetAllDialogs(names: *mut *mut c_char, n: c_int) -> c_int;
// pub fn IupGetName(ih: *mut Ihandle) -> *mut c_char;

// pub fn IupSetAttributeHandle(ih: *mut Ihandle, name: *const c_char, ih_named: *mut Ihandle);
// pub fn IupGetAttributeHandle(ih: *mut Ihandle, name: *const c_char) -> *mut Ihandle;

// pub fn IupGetClassName(ih: *mut Ihandle) -> *mut c_char;
// pub fn IupGetClassType(ih: *mut Ihandle) -> *mut c_char;
// pub fn IupGetAllClasses(names: *mut *mut c_char, n: c_int) -> c_int;
// pub fn IupGetClassAttributes(classname: *const c_char, names: *mut *mut c_char, n: c_int) -> c_int;
// pub fn IupGetClassCallbacks(classname: *const c_char, names: *mut *mut c_char, n: c_int) -> c_int;
// pub fn IupSaveClassAttributes(ih: *mut Ihandle);
// pub fn IupCopyClassAttributes(src_ih: *mut Ihandle, dst_ih: *mut Ihandle);
// pub fn IupSetClassDefaultAttribute(classname: *const c_char, name: *const c_char, value: *const c_char);
// pub fn IupClassMatch(ih: *mut Ihandle, classname: *const c_char) -> c_int;

// pub fn IupCreate(classname: *const c_char) -> *mut Ihandle;
// pub fn IupCreatev(classname: *const c_char, params: *mut *mut c_void) -> *mut Ihandle;
// pub fn IupCreatep(classname: *const c_char, first: *mut c_void, ...) -> *mut Ihandle;

/************************************************************************/
/*                        Elements                                      */
/************************************************************************/
pub fn fill() -> Ihandle {
    unsafe { Ihandle::from_ptr(iup_sys::IupFill()) }
}

// pub fn IupRadio(child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupVbox(child: *mut Ihandle, ...) -> *mut Ihandle;

pub fn vboxv(child: &[Ihandle]) -> Ihandle {
    let mut v = slice_to_vvec(child);
    unsafe { Ihandle::from_ptr(iup_sys::IupVboxv(v.as_mut_ptr())) }
}

// pub fn IupZbox(child: *mut Ihandle, ...) -> *mut Ihandle;
// pub fn IupZboxv(children: *mut *mut Ihandle) -> *mut Ihandle;
// pub fn IupHbox(child: *mut Ihandle, ...) -> *mut Ihandle;

pub fn hboxv(child: &[Ihandle]) -> Ihandle {
    let mut v = slice_to_vvec(child);
    unsafe { Ihandle::from_ptr(iup_sys::IupHboxv(v.as_mut_ptr())) }
}

// pub fn IupNormalizer(ih_first: *mut Ihandle, ...) -> *mut Ihandle;
// pub fn IupNormalizerv(ih_list: *mut *mut Ihandle) -> *mut Ihandle;

// pub fn IupCbox(child: *mut Ihandle, ...) -> *mut Ihandle;
// pub fn IupCboxv(children: *mut *mut Ihandle) -> *mut Ihandle;
// pub fn IupSbox(child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupSplit(child1: *mut Ihandle, child2: *mut Ihandle) -> *mut Ihandle;
// pub fn IupScrollBox(child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupGridBox(child: *mut Ihandle, ...) -> *mut Ihandle;
// pub fn IupGridBoxv(children: *mut *mut Ihandle) -> *mut Ihandle;
// pub fn IupExpander(child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupDetachBox(child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupBackgroundBox(child: *mut Ihandle) -> *mut Ihandle;

// pub fn IupFrame(child: *mut Ihandle) -> *mut Ihandle;

// pub fn IupImage(width: c_int, height: c_int, pixmap: *const c_uchar) -> *mut Ihandle;
// pub fn IupImageRGB(width: c_int, height: c_int, pixmap: *const c_uchar) -> *mut Ihandle;
// pub fn IupImageRGBA(width: c_int, height: c_int, pixmap: *const c_uchar) -> *mut Ihandle;

// pub fn IupItem(title: *const c_char, action: *const c_char) -> *mut Ihandle;
// pub fn IupSubmenu(title: *const c_char, child: *mut Ihandle) -> *mut Ihandle;
// pub fn IupSeparator() -> *mut Ihandle;
// pub fn IupMenu(child: *mut Ihandle, ...) -> *mut Ihandle;
// pub fn IupMenuv(children: *mut *mut Ihandle) -> *mut Ihandle;

pub fn button(text: &str) -> Ihandle {
    let text_c = CString::new(text).unwrap();
    unsafe { Ihandle::from_ptr(iup_sys::IupButton(text_c.as_ptr(), ptr::null())) }
}

// pub fn IupCanvas(action: *const c_char) -> *mut Ihandle;

pub fn dialog(child: Ihandle) -> Ihandle {
    unsafe { Ihandle::from_ptr(iup_sys::IupDialog(child.ptr)) }
}

// pub fn IupUser() -> *mut Ihandle;

pub fn label(text: &str) -> Ihandle {
    let text_c = CString::new(text).unwrap();
    unsafe { Ihandle::from_ptr(iup_sys::IupLabel(text_c.as_ptr())) }
}

// pub fn IupList(action: *const c_char) -> *mut Ihandle;

pub fn text() -> Ihandle {
    let action: *const c_char = ptr::null();
    unsafe { Ihandle::from_ptr(iup_sys::IupText(action)) }
}

// pub fn IupMultiLine(action: *const c_char) -> *mut Ihandle;
// pub fn IupToggle(title: *const c_char, action: *const c_char) -> *mut Ihandle;
// pub fn IupTimer() -> *mut Ihandle;
// pub fn IupClipboard() -> *mut Ihandle;
// pub fn IupProgressBar() -> *mut Ihandle;
// pub fn IupVal(_type: *const c_char) -> *mut Ihandle;
// pub fn IupTabs(child: *mut Ihandle, ...) -> *mut Ihandle;
// pub fn IupTabsv(children: *mut *mut Ihandle) -> *mut Ihandle;
// pub fn IupTree() -> *mut Ihandle;
// pub fn IupLink(url: *const c_char, title: *const c_char) -> *mut Ihandle;

/************************************************************************/
/*                      Utilities                                       */
/************************************************************************/

/* IupImage utility */
// pub fn IupSaveImageAsText(ih: *mut Ihandle, file_name: *const c_char, format: *const c_char, name: *const c_char) -> c_int;

/* IupText and IupScintilla utilities */
// pub fn IupTextConvertLinColToPos(ih: *mut Ihandle, lin: c_int, col: c_int, pos: *mut c_int);
// pub fn IupTextConvertPosToLinCol(ih: *mut Ihandle, pos: c_int, lin: *mut c_int, col: *mut c_int);

/* IupTree utilities */
// pub fn IupTreeSetUserId(ih: *mut Ihandle, id: c_int, userid: *mut c_void) -> c_int;
// pub fn IupTreeGetUserId(ih: *mut Ihandle, id: c_int) -> *mut c_void;
// pub fn IupTreeGetId(ih: *mut Ihandle, userid: *mut c_void) -> c_int;
// pub fn IupTreeSetAttributeHandle(ih: *mut Ihandle, name: *const c_char, id: c_int, ih_named: *mut Ihandle);

/************************************************************************/
/*                      Pre-definided dialogs                           */
/************************************************************************/
// pub fn IupFileDlg() -> *mut Ihandle;
// pub fn IupMessageDlg() -> *mut Ihandle;
// pub fn IupColorDlg() -> *mut Ihandle;
// pub fn IupFontDlg() -> *mut Ihandle;
// pub fn IupProgressDlg() -> *mut Ihandle;

// pub fn IupGetFile(arq: *mut c_char) -> c_int;
// pub fn IupMessage(title: *const c_char, msg: *const c_char);
// pub fn IupMessagef(title: *const c_char, format: *const c_char, ...);
// pub fn IupAlarm(title: *const c_char, msg: *const c_char, b1: *const c_char, b2: *const c_char, b3: *const c_char) -> c_int;
// pub fn IupScanf(format: *const c_char, ...) -> c_int;
// pub fn IupListDialog(_type: c_int, title: *const c_char, size: c_int, list: *mut *const c_char, op: c_int, max_col: c_int, max_lin: c_int, marks: *mut c_int) -> c_int;
// pub fn IupGetText(title: *const c_char, text: *mut c_char) -> c_int;
// pub fn IupGetColor(x: c_int, y: c_int, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar) -> c_int;

// pub fn IupGetParam(title: *const c_char, action: Iparamcb, user_data: *mut c_void, format: *const c_char, ...) -> c_int;
// pub fn IupGetParamv(title: *const c_char, action: Iparamcb, user_data: *mut c_void, format: *const c_char, param_count: c_int, param_extra: c_int, param_data: *mut *mut c_void) -> c_int;

// pub fn IupLayoutDialog(dialog: *mut Ihandle) -> *mut Ihandle;
// pub fn IupElementPropertiesDialog(elem: *mut Ihandle) -> *mut Ihandle;


pub fn load<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    // The following line uses ok_or_else instead of ok_or to lazyly make a string
    // instead of making it straight away.
    let str = try!(path.to_str().ok_or_else(|| "Failed to convert Path to string".to_string()));
    let str_c = CString::new(str).unwrap();

    match unsafe { iup_sys::IupLoad(str_c.as_ptr()) } {
        err if err.is_null() => Ok(()),
        err => unsafe {
            Err(String::from_utf8_lossy(CStr::from_ptr(err).to_bytes()).to_string())
        },
    }
}

pub fn load_buffer(buf: &str) -> Result<()> {

    let str_c = CString::new(buf).unwrap();

    match unsafe { iup_sys::IupLoadBuffer(str_c.as_ptr()) } {
        err if err.is_null() => Ok(()),
        err => unsafe {
            Err(String::from_utf8_lossy(CStr::from_ptr(err).to_bytes()).to_string())
        },
    }
}

