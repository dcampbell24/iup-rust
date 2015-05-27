// TODO CRATE DOC
use iup_sys;
use std::ptr;

use Element;

// TODO to use Option (new(Option)) or new/new_empty instead?
// notice None doesn't seem to quite infer the type of `H` (compile time error).
// same question for Button, Label and others.

// TODO new(child) or with(child)?
// same question for other containers!

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

impl_element!(Dialog);
