// TODO MOD DOC
use iup_sys;
use std::ptr;

use Element;

/// See the [IUP Text Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iuptext.html
pub struct Text(*mut iup_sys::Ihandle);

impl Text {
	///Creates a Text with no predefined text.
	pub fn new() -> Text {
		unsafe { Text::from_raw(iup_sys::IupText(ptr::null_mut())) }
	}
}

impl_element!(Text);
impl ::callback::Action for Text {}
