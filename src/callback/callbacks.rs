use libc::{c_char, c_int};
use std::path::PathBuf;

// http://sourceforge.net/p/iup/iup/2620/tree//trunk/iup/include/iupcbs.h

impl_callback! {
    let name = "IDLE_ACTION";
    extern fn listener() -> CallbackReturn;
    /// Action generated when there are no events or messages to be processed.
    ///
    /// Often used to perform background operations.
    pub fn set_idle<F: Callback()>(cb: F);
    /// Removes a previosly set up idle_action callback.
    pub fn remove_idle() -> Option<Box<_>>;
}

// Common Callbacks
// ----------------------------

// This is the common version of the ACTION callback, any so called ACTION that does not
// have the `(*mut iup_sys::Ihandle)` signature should be another trait.
impl_callback! {
    /// Action generated when the element is activated. Affects each element differently.
    /// 
    /// See the documentation of the `Self` object for the effect of this callback on it.
    pub trait Action where Self: Element {
        let name = "ACTION";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_action<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_action(&mut self) -> Option<Box<_>>;
    }
}

// Note on using LDESTROY_CB instead of DESTROY_CB:
//
//    IUP calls LDESTROY_CB and then calls DESTROY_CB. It was thought LDESTROY_CB should be used
//    for our binding to free things up, but we're doing it the other way around for simplicity.
//
//    The binding free needs to be called after LDESTROY_CB (i.e. at DESTROY_CB) because
//    the LDESTROY_CB event call depends on the binding boxed data to work properly.
impl_callback! {
    /// Called right before an element is destroyed.
    pub trait DestroyCb where Self: Element {
        let name = "LDESTROY_CB";   // See comments above for reason behind LDESTROY_CB.
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_destroy_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_destroy_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    /// Called right after an element is mapped and its attributes updated in `Element::map`.
    ///
    /// When the element is a dialog, it is called after the layout is updated. For all other
    /// elements is called before the layout is updated, so the element current size will still
    /// be 0x0 during MAP_CB.
    pub trait MapCb where Self: Element {
        let name = "MAP_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_map_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_map_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    /// Called right before an element is unmapped.
    pub trait UnmapCb where Self: Element {
        let name = "UNMAP_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_unmap_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_unmap_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    /// Action generated when an element is given keyboard focus.
    ///
    /// This callback is called after the `KillFocusCb` of the element that loosed the focus.
    /// The IupGetFocus (TODO) function during the callback returns the element that loosed the focus.
    pub trait GetFocusCb where Self: Element {
        let name = "GETFOCUS_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_getfocus_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_getfocus_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    /// Action generated when an element loses keyboard focus.
    ///
    /// This callback is called before the `GetFocusCb` of the element that gets the focus.
    ///
    /// While processing this message, do not make any function calls that display or activate a
    /// window. This causes the thread to yield control and can cause the application to stop
    /// responding to messages.
    pub trait KillFocusCb where Self: Element {
        let name = "KILLFOCUS_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_killfocus_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_killfocus_cb(&mut self) -> Option<Box<_>>;                
    }
}

impl_callback! {
    /// Action generated when the mouse enters the native element.
    ///
    /// When the cursor is moved from one element to another, the call order in all platforms will
    /// be first the `LeaveWindowCb` callback of the old control followed by the `EnterWindowCb`
    /// callback of the new control.
    pub trait EnterWindowCb where Self: Element {
        let name = "ENTERWINDOW_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_enterwindow_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_enterwindow_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    /// Action generated when the mouse leaves the native element.
    ///
    /// When the cursor is moved from one element to another, the call order in all platforms will
    /// be first the `LeaveWindowCb` callback of the old control followed by the `EnterWindowCb`
    /// callback of the new control.
    pub trait LeaveWindowCb where Self: Element {
        let name = "LEAVEWINDOW_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_leavewindow_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_leavewindow_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    /// Action generated when the user press F1 at a control.
    ///
    /// `CallbackReturn::Close` will be processed.
    pub trait HelpCb where Self: Element {
        let name = "HELP_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_help_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_help_cb(&mut self) -> Option<Box<_>>;
    }
}

// Other Callbacks
// ----------------------------

impl_callback! {
    /// Action called when a file is *dropped* into the control.
    ///
    /// When several files are dropped at once, the callback is called several times, once for
    /// each file. 
    ///
    /// If defined after the element is mapped then the attribute DROPFILESTARGET must be set
    /// to YES.
    ///
    /// The third parameter of the callback is the number index of the dropped file. If several
    /// files are dropped, it is the index of the dropped file starting from *total-1* to *0*.
    /// The fourth and fifth parameters are x,y coordinate of the point where the user
    /// released the mouse button.
    ///
    /// if `CallbackReturn::Ignore`  is returned the callback will **not** be called for the
    /// next dropped files, and the processing of dropped files will be interrupted.
    ///
    #[doc="\\[Windows and GTK Only\\] (GTK 2.6)"]
    pub trait DropFilesCb where Self: Element {
        let name = "DROPFILES_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, filename: *const c_char,
                           num: c_int, x: c_int, y: c_int) -> CallbackReturn;
        fn set_dropfiles_cb<F: Callback(Self, PathBuf, usize, i32, i32)>(&mut self, cb: F) -> Self;
        fn remove_dropfiles_cb(&mut self) -> Option<Box<_>>;
        // TODO should it be plural (dropfiles_cb) just like IUP or singular (dropfile_cb)?
    }
}

