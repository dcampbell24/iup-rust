use iup_sys;
use std::ptr;
use std::ffi::CString;

use Element;

/// See the [IUP Toggle Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iuptoggle.html
pub struct Toggle(*mut iup_sys::Ihandle);

impl Toggle {
    /// Creates a toggle with no text.
    pub fn new() -> Toggle {
        unsafe { Toggle::from_raw(iup_sys::IupToggle(ptr::null_mut(), ptr::null_mut())) }
    }

    /// Creates a toggle with the specified text.
    pub fn with_title<S: Into<String>>(title: S) -> Toggle {
        let ctitle = CString::new(title.into()).unwrap();
        unsafe { Toggle::from_raw(iup_sys::IupToggle(ctitle.as_ptr(), ptr::null_mut())) }
    }
}

impl_widget!(Toggle, "toggle");
impl ::callback::MapCb for Toggle {}
impl ::callback::UnmapCb for Toggle {}
impl ::callback::GetFocusCb for Toggle {}
impl ::callback::KillFocusCb for Toggle {}
impl ::callback::EnterWindowCb for Toggle {}
impl ::callback::LeaveWindowCb for Toggle {}
impl ::callback::HelpCb for Toggle {}
// TODO impl K_ callbacks when it's implemented.

/// Called after the value was interactively changed by the user.
///
/// Called after the ACTION callback, but under the same context.
impl ::callback::ValueChangedCb for Toggle {}

/// See the `ToggleAction` documentation.
impl self::ToggleAction for Toggle {}
impl_callback! {
    #[doc="Action generated when the toggle's state (on/off) was changed."]
    #[doc=""]
    #[doc="The callback boolean parameter represents the state the toggle was switched to."]
    #[doc=""]
    #[doc="`CallbackReturn::Close` will be processed"]
    pub trait ToggleAction where Self: Element {
        let name = "ACTION";
        extern fn listener(ih: *mut iup_sys::Ihandle, state: c_int) -> CallbackReturn;
        fn set_action<F: Callback(Self, bool)>(&mut self, cb: F) -> Self;
        fn remove_action(&mut self) -> Option<Box<_>>;
    }
}
