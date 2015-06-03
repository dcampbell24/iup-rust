use iup_sys;
use std::ptr;

use Element;

/// See the [IUP Frame Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupframe.html
pub struct Frame(*mut iup_sys::Ihandle);

impl Frame {
    /// Creates a frame with a child element.
    pub fn new<E: Element>(child: E) -> Frame {
        unsafe { Frame::from_raw(iup_sys::IupFrame(child.raw())) }
    }

    /// Creates a frame with no elements.
    pub fn new_empty() -> Frame {
        unsafe { Frame::from_raw(iup_sys::IupFrame(ptr::null_mut())) }
    }
}

impl_widget_container!(Frame, "frame");
impl ::callback::MapCb for Frame {}
impl ::callback::UnmapCb for Frame {}
