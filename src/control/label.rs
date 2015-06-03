use iup_sys;
use std::ptr;
use std::ffi::CString;

use Element;
use Orientation;

/// See the [IUP Label Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iuplabel.html
pub struct Label(*mut iup_sys::Ihandle);

impl Label {
    /// Creates a label with no predefined text, image or separator.
    pub fn new() -> Label {
        unsafe { Label::from_raw(iup_sys::IupLabel(ptr::null_mut())) }
    }

    /// Creates a label separator in the specified orientation.
    pub fn new_separator(orient: Orientation) -> Label {
        Label::new().set_attrib_data("SEPARATOR", orient.as_cstr() as *const _)
    }

    // TODO with_image(...)

    /// Creates a label interface element which displays a text.
    pub fn with_title<S: Into<String>>(label: S) -> Label {
        let clabel = CString::new(label.into()).unwrap();
        unsafe { Label::from_raw(iup_sys::IupLabel(clabel.as_ptr())) }
    }
}

impl_widget!(Label, "label");
impl ::callback::MapCb for Label {}
impl ::callback::UnmapCb for Label {}
impl ::callback::EnterWindowCb for Label {}
impl ::callback::LeaveWindowCb for Label {}

/// Action generated when any mouse button is pressed or released.
impl ::callback::button::ButtonCb for Label {}

/// Action generated when one or more files are dropped in the element.
impl ::callback::DropFilesCb for Label {}

// TODO impl future DragSource and DragTarget traits.
