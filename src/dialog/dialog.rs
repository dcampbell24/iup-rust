use iup_sys;
use libc::{c_char, c_int};
use std::ptr;

use Element;
use callback::IntoRust;
use callback::button::{MouseButton, MouseButtonState};


/// Dialog elements manages user interaction with the interface elements.
/// For any interface element to be shown, it must be encapsulated in a dialog.
///
/// See the [IUP Dialog Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupdialog.html
pub struct Dialog(*mut iup_sys::Ihandle);

impl Dialog {
    /// Creates a dialog with a child element.
    pub fn new<E: Element>(child: E) -> Dialog {
        unsafe { Dialog::from_raw(iup_sys::IupDialog(child.raw())) }
    }

    /// Creates a dialog with no elements.
    pub fn new_empty() -> Dialog {
        unsafe { Dialog::from_raw(iup_sys::IupDialog(ptr::null_mut())) }
    }
}

impl_dialog!(Dialog, "dialog");
impl ::callback::MapCb for Dialog {}
impl ::callback::UnmapCb for Dialog {}
impl ::callback::GetFocusCb for Dialog {}
impl ::callback::KillFocusCb for Dialog {}
impl ::callback::EnterWindowCb for Dialog {}
impl ::callback::LeaveWindowCb for Dialog {}
impl ::callback::HelpCb for Dialog {}
// TODO impl K_ callbacks when it's implemented.

/// Called right before the dialog is closed.
impl ::callback::CloseCb for Dialog {}

/// Action generated when one or more files are dropped in the dialog.
impl ::callback::DropFilesCb for Dialog {}

/// Called after the dialog was moved on screen.
///
/// The coordinates are the same as the SCREENPOSITION attribute.
impl ::callback::MoveCb for Dialog {}

/// Action generated when the dialog size is changed.
///
/// This action is also generated when the dialog is mapped, after the map and before the show.
///
/// If returns `CallbackReturn::Ignore` the dialog layout is **not** recalculated.
impl ::callback::ResizeCb for Dialog {}

/// See the `CopyDataCb` documentation.
impl self::CopyDataCb for Dialog {}

/// See the `MdiActivateCb` documentation.
impl self::MdiActivateCb for Dialog {}

/// See the `ShowCb` documentation.
impl self::ShowCb for Dialog {}

/// See the `TrayClickCb` documentation.
impl self::TrayClickCb for Dialog {}



impl_callback! {
    #[doc="[Windows Only]: Called at the first instance, when a second instance is running."]
    #[doc=""]
    #[doc="Must set the global attribute SINGLEINSTANCE to be called."]
    #[doc=""]
    #[doc="The `String` parameter is the command line of the second instance. "]
    pub trait CopyDataCb where Self: Element {
        let name = "COPYDATA_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, cmdline: *const c_char, size: c_int) -> CallbackReturn;
        fn set_copydata_cb<F: Callback(Self, String)>(&mut self, cb: F) -> Self;
        fn remove_copydata_cb(&mut self) -> Option<Box<_>>;

        fn resolve_args(elem: Self, cmdline: *const c_char, _size: c_int) -> (Self, String) {
            (elem, string_from_cstr!(cmdline))
        }
    }
}

impl_callback! {
    #[doc="[Windows Only]: Called when a MDI child window is activated."]
    #[doc=""]
    #[doc="Only the MDI child receive this message. It is not called when the child is shown for the"]
    #[doc="first time."]
    #[doc=""]
    #[doc="See the [IUP Dialog Documentation][1] for information on MDI dialogs."]
    #[doc="[1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupdialog.html"]
    pub trait MdiActivateCb where Self: Element {
        let name = "MDIACTIVATE_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_mdiactivate_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_mdiactivate_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Called right after the dialog is showed, hidden, maximized, minimized or restored from"]
    #[doc="minimized/maximized."]
    #[doc=""]
    #[doc="This callback is called when those actions were performed by the user or programmatically"]
    #[doc="by the application."]
    #[doc=""]
    #[doc="`CallbackReturn::Close` will be processed."]
    pub trait ShowCb where Self: Element {
        let name = "SHOW_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, state: c_int) -> CallbackReturn;
        fn set_move_cb<F: Callback(Self, ShowState)>(&mut self, cb: F) -> Self;
        fn remove_move_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="[Windows and GTK Only]: Called right after the mouse button is pressed or released over"]
    #[doc="the tray icon."]
    #[doc=""]
    #[doc="The fourth callback parameter is a bool indicating whether the button was double pressed."]
    #[doc=""]
    #[doc="`CallbackReturn::Close` will be processed."]
    pub trait TrayClickCb where Self: Element {
        let name = "TRAYCLICK_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, but: c_int, pressed: c_int, dclick: c_int) -> CallbackReturn;
        fn set_trayclick_cb<F: Callback(Self, MouseButton, MouseButtonState, bool)>(&mut self, cb: F) -> Self;
        fn remove_trayclick_cb(&mut self) -> Option<Box<_>>;

        fn resolve_args(elem: Self, but: c_int, pressed: c_int, dclick: c_int)
                                             -> (Self, MouseButton, MouseButtonState, bool) {
            (elem, MouseButton::from_id(but), pressed.into_rust(), dclick.into_rust())
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ShowState {
    Hide,
    Show,
    /// Was minimized or maximized.
    Restore,
    Minimize,
    /// **Note:** Not received in Motif when activated from the maximize button
    Maximize,
}

impl IntoRust<ShowState> for c_int {
    fn into_rust(self) -> ShowState {
        match self {
            iup_sys::IUP_HIDE => ShowState::Hide,
            iup_sys::IUP_SHOW => ShowState::Show,
            iup_sys::IUP_RESTORE => ShowState::Restore,
            iup_sys::IUP_MINIMIZE => ShowState::Minimize,
            iup_sys::IUP_MAXIMIZE => ShowState::Maximize,
            _ => unreachable!(),
        }
    }
}
