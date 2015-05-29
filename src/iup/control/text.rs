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

impl_element!(Text, "text");
impl ::callback::MapCb for Text {}
impl ::callback::UnmapCb for Text {}
impl ::callback::GetFocusCb for Text {}
impl ::callback::KillFocusCb for Text {}
impl ::callback::EnterWindowCb for Text {}
impl ::callback::LeaveWindowCb for Text {}
impl ::callback::HelpCb for Text {}
// TODO impl K_ callbacks when it's implemented.

// TODO TextAction callback instead of Action. Sign: `(Self, Option<std::char>, &str|String)
// (though is the `int c` parameter really a unicode thing?)

/// Action generated when any mouse button is pressed or released.
///
/// Use `Element::convert_xy_to_pos` to convert (x,y) coordinates in character positioning.
impl ::callback::button::ButtonCb for Text {}
