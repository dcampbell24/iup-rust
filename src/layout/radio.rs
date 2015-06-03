use iup_sys;
use std::ptr;

use Element;

/// A void container for grouping mutual exclusive toggles.
/// Only one of its descendent toggles will be active at a time.
/// The toggles can be at any composition.
///
/// See the [IUP Radio Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupradio.html
pub struct Radio(*mut iup_sys::Ihandle);

impl Radio {
    /// Creates a radio to wrap toggles.
    ///
    /// The child is usually a `VBox` or `HBox` containing the toggles.
    pub fn new<E: Element>(child: E) -> Radio {
        unsafe { Radio::from_raw(iup_sys::IupRadio(child.raw())) }
    }

    /// Creates a radio with no wrapped content.
    pub fn new_empty() -> Radio {
        unsafe { Radio::from_raw(iup_sys::IupRadio(ptr::null_mut())) }
    }
}

impl_widget_container!(Radio, "radio");
