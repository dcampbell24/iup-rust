// TODO MOD DOC
use iup_sys;
use std::ptr;
use std::ffi::CString;

use Element;

/// See the [IUP Button Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupbutton.html
pub struct Button(*mut iup_sys::Ihandle);

impl Button {
	/// Creates a Button with no text.
	pub fn new() -> Button {
		unsafe { Button::from_raw(iup_sys::IupButton(ptr::null_mut(), ptr::null_mut())) }
	}

	/// Creates a Button with the specified text.
	pub fn with_title<S: Into<String>>(title: S) -> Button {
		let clabel = CString::new(title.into()).unwrap();
		unsafe { Button::from_raw(iup_sys::IupButton(clabel.as_ptr(), ptr::null_mut())) }
	}

	// TODO with_image
}

impl_element!(Button, "button");

/// Action generated when the button 1 (usually left) is selected.
///
/// This callback is called only after the mouse is released and when it is released
/// inside the button area.
///
/// `CallbackReturn::Close` will be processed.
impl ::callback::Action for Button {}
