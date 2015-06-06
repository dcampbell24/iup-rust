//! See [IUP Dialogs][1].
//! [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/dialogs.html

use iup_sys;
use libc::c_int;
use std::result::Result;

use element::{Element, Widget, Container};

macro_rules! impl_dialog {
    ($ty_path:path, $classname:expr) => {
        impl_widget_container!($ty_path, $classname);
        impl $crate::dialog::DialogElement for $ty_path {}
    }
}

pub mod dialog;
pub mod alarm;
pub mod message;
pub mod file;

pub use self::dialog::{Dialog, ShowState, CopyDataCb, MdiActivateCb, ShowCb, TrayClickCb};
pub use self::alarm::{AlarmButton, alarm};
pub use self::message::{MessageDlg, message};
pub use self::file::{FileDlg};

// An dialog is a top-level container.
pub trait DialogElement : Element + Widget + Container {
	/// Displays a dialog in a given position on the screen.
	///
	/// Will call `Widget::map` for the element.
	///
	/// This function can be executed more than once for the same dialog. This will make the
	/// dialog be placed above all other dialogs in the application, changing its Z-order, and
	/// update its position and/or size on screen.
    ///
    /// The string wrapped in `Err` may be meaningless, it is this way so that the returned value
    /// of this call can be passed directly to the closure return of `with_iup`.
    ///
	/// # Panics
	/// Panics if `x` is either `Bottom` or `Top` or if `y` is either `Left` or `Right`.
	fn showxy(&mut self, x: DialogPos, y: DialogPos) -> Result<(), String> {
        match unsafe { iup_sys::IupShowXY(self.raw(), x.to_raw_x(), y.to_raw_y()) } {
            iup_sys::IUP_NOERROR => Ok(()),
            iup_sys::IUP_ERROR => Err("showxy:IUP_ERROR".into()),
            _ => unreachable!(),
        }
    }

	/// Shows a dialog or menu and restricts user interaction only to the specified element.
	///
	/// It is equivalent of creating a *modal* dialog is some toolkits.
	///
	/// If another dialog is shown after `popup` using `show`, then its interaction will not be
	/// inhibited. Every `popup` call creates a new popup level that inhibits all previous dialogs
	/// interactions, but does not disable new ones. IMPORTANT: The popup levels must be closed in
	/// the reverse order they were created or unpredictable results will occur.
	///
	/// For a dialog this function will only return the control to the application after a callback
	/// returns `CallbackReturn::Close`, IupExitLoop (TODO) is called, or when the popup dialog is 
	/// hidden, for example using `Widget::hide`. For a menu it returns automatically after a menu
	/// item is selected. IMPORTANT: If a menu item callback returns `CallbackReturn::Close`,
	/// it will ends the current popup level dialog.
	///
	/// # Panics
	/// Panics if `x` is either `Bottom` or `Top` or if `y` is either `Left` or `Right`.
	fn popup(&mut self, x: DialogPos, y: DialogPos) -> Result<Self, Self> {
	    match unsafe { iup_sys::IupPopup(self.raw(), x.to_raw_x(), y.to_raw_y()) } {
	        iup_sys::IUP_NOERROR => Ok(*self),
	        iup_sys::IUP_ERROR => Err(*self),
	        // This should NEVER happen as DialogElement is supposed to be impl'ed only by dialogs.
	        iup_sys::IUP_INVALID => panic!("`DialogElement::popup` called on a non-dialog!"),
	        _ => unreachable!(),
	    }
	}
}

/// The position a dialog should be positioned.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DialogPos {
	/// Positions the element at the specified coordinate.
    At(i32),
    /// Use the current position of the dialog. Not valid for menus.
    /// This should be used most of the time.
    Current,
    /// Centers the element on the screen
    Center,
    /// Centralizes the dialog relative to its parent.
    CenterParent,
    /// Positions the element on the mouse cursor.
    MousePos,
    /// Positions the element on the left corner of the screen. Valid only for the x axis.
    Left,
	/// Positions the element on the right corner of the screen. Valid only for the x axis.
    Right,
    /// Positions the element on the top of the screen. Valid only for the y axis.
    Top,
	/// Positions the element on the bottom of the screen. Valid only for the y axis.
    Bottom,   
}

impl DialogPos {
    fn to_raw(&self) -> c_int {
    	use self::DialogPos::*;
        match *self {
            At(i) => i,
            Top => iup_sys::IUP_TOP,
            Bottom => iup_sys::IUP_BOTTOM,
            Left => iup_sys::IUP_LEFT,
            Right => iup_sys::IUP_RIGHT,
            Current => iup_sys::IUP_CURRENT,
            MousePos => iup_sys::IUP_MOUSEPOS,
            Center => iup_sys::IUP_CENTER,
            CenterParent => iup_sys::IUP_CENTERPARENT,
        }
    }

    fn to_raw_x(&self) -> c_int {
    	use self::DialogPos::*;
    	assert!(*self != Top && *self != Bottom);
    	self.to_raw()
    }

    fn to_raw_y(&self) -> c_int {
    	use self::DialogPos::*;
    	assert!(*self != Right && *self != Left);
    	self.to_raw()
    }
}
