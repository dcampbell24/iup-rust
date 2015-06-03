//! Mouse button presses callback.
use iup_sys;
use std::fmt;
use libc::{c_char, c_int};
use callback::IntoRust;

/// Mouse buttons.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MouseButton {
    /// Left mouse button.
    Button1,
    /// Middle mouse button.
    Button2,
    /// Right mouse button.
    Button3,
    /// Additional mouse button.
    Button4,
    /// Additional mouse button.
    Button5,
}

impl MouseButton {
    #[doc(hidden)]
    pub fn from_id(id: c_int) -> MouseButton {
        match id {
            1 => MouseButton::Button1,
            2 => MouseButton::Button2,
            3 => MouseButton::Button3,
            4 => MouseButton::Button4,
            5 => MouseButton::Button5,
            _ => unreachable!(),
        }
    }
}

/// Specifies what happened to the mouse button in the `ButtonCb`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MouseButtonState {
    Released,
    Pressed,
}

// TODO a way to unallow the user to move/copy instances of this type as the char buffer will
// be invalid after the callback exits.
/// The state of mouse buttons and some keyboard buttons.
pub struct KeyStates(*const c_char);

impl KeyStates {
    /// Whether this state have a SHIFT key pressed.
    #[inline(always)]
    pub fn is_shift(&self) -> bool {
        unsafe { iup_sys::iup_isshift(self.0) }
    }
    /// Whether this state have a CONTROL key pressed.
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        unsafe { iup_sys::iup_iscontrol(self.0) }
    }
    /// Whether this state have a ALT key pressed.
    #[inline(always)]
    pub fn is_alt(&self) -> bool {
        unsafe { iup_sys::iup_isalt(self.0) }
    }
    /// Whether this state have the system key pressed.
    ///
    /// The system key in Windows is the *Windows key* and in Mac is the *Apple key*.
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        unsafe { iup_sys::iup_issys(self.0) }
    }
    /// Whether this state have the specified button in the callback doubly pressed.
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        unsafe { iup_sys::iup_isdouble(self.0) }
    }
    /// Whether this state have the left mouse button pressed.
    #[inline(always)]
    pub fn is_button1(&self) -> bool {
        unsafe { iup_sys::iup_isbutton1(self.0) }
    }
    /// Whether this state have the middle mouse button pressed.
    #[inline(always)]
    pub fn is_button2(&self) -> bool {
        unsafe { iup_sys::iup_isbutton2(self.0) }
    }
    /// Whether this state have the right mouse button pressed.
    #[inline(always)]
    pub fn is_button3(&self) -> bool {
        unsafe { iup_sys::iup_isbutton3(self.0) }
    }
    /// Whether this state have the additional mouse button 1 pressed.
    #[inline(always)]
    pub fn is_button4(&self) -> bool {
        unsafe { iup_sys::iup_isbutton4(self.0) }
    }
    /// Whether this state have the additional mouse button 2 pressed.
    #[inline(always)]
    pub fn is_button5(&self) -> bool {
        unsafe { iup_sys::iup_isbutton5(self.0) }
    }
}

impl fmt::Debug for KeyStates {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!(
            "KeyStates(Shift={}, Control={}, Alt={}, Sys={}, Double={},\
             Button1={}, Button2={}, Button3={}, Button4={}, Button5={})",
            self.is_shift(), self.is_control(), self.is_alt(), self.is_sys(), self.is_double(),
            self.is_button1(), self.is_button2(), self.is_button3(), self.is_button4(), self.is_button5()
        ))
    }
}

impl IntoRust<KeyStates> for *mut c_char {
    fn into_rust(self) -> KeyStates {
        KeyStates(self)
    }
}

impl IntoRust<MouseButton> for c_int {
    fn into_rust(self) -> MouseButton {
        match self {
            iup_sys::IUP_BUTTON1 => MouseButton::Button1,
            iup_sys::IUP_BUTTON2 => MouseButton::Button2,
            iup_sys::IUP_BUTTON3 => MouseButton::Button3,
            iup_sys::IUP_BUTTON4 => MouseButton::Button4,
            iup_sys::IUP_BUTTON5 => MouseButton::Button5,
            _ => unreachable!(),
        }
    }
}

impl IntoRust<MouseButtonState> for c_int {
    fn into_rust(self) -> MouseButtonState {
        if self != 0 { MouseButtonState::Pressed } else { MouseButtonState::Released }
    }
}


impl_callback! {
    /// Action generated when a mouse button is pressed or released.
    ///
    /// The `Button` parameter identifies the activated mouse button that triggered the action.
    ///
    /// The `i32` parameters are the x,y position in the canvas where the event has occurred,
    /// in pixels.
    ///
    /// The `KeyStates` parameter is the state of the mouse buttons and some keyboard keys at
    //// the moment the event is generated.
    ///
    /// `CallbackReturn::Close` will be processed. On some controls if `CallbackReturn::Ignore`
    /// is returned the action is ignored *(this is system dependent)*.
    ///
    /// This callback can be used to customize a button behavior. For a standard button behavior
    /// use the `Action` callback.
    ///
    /// [Learn more](http://webserver2.tecgraf.puc-rio.br/iup/en/call/iup_button_cb.html).
    pub trait ButtonCb where Self: Element {
        let name = "BUTTON_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, button: c_int, pressed: c_int,
                           x: c_int, y: c_int, status: *mut c_char) -> CallbackReturn;
        fn set_button_cb<F: Callback(Self, MouseButton, MouseButtonState, i32, i32, KeyStates)>(&mut self, cb: F) -> Self;
        fn remove_button_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    /// Action generated when the mouse moves.
    ///
    /// The `i32` parameters are the x,y position in the canvas where the event has occurred,
    /// in pixels.
    ///
    /// The `KeyStates` parameter is the state of the mouse buttons and some keyboard keys at
    //// the moment the event is generated.
    pub trait MotionCb where Self: Element {
        let name = "MOTION_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, x: c_int, y: c_int, status: *mut c_char) -> CallbackReturn;
        fn set_motion_cb<F: Callback(Self, i32, i32, KeyStates)>(&mut self, cb: F) -> Self;
        fn remove_motion_cb(&mut self) -> Option<Box<_>>;
    }
}

