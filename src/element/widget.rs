//! Graphical user interface commons.
use iup_sys;
use std::result::Result;

use element::{Element, Node};

macro_rules! impl_widget {
    ($ty_path:path, $classname:expr) => {
        impl_element!($ty_path, $classname);
        impl $crate::element::Widget for $ty_path {}
        impl $crate::element::Node for $ty_path {}
    }
}

macro_rules! impl_widget_container {
    ($ty_path:path, $classname:expr) => {
        impl_widget!($ty_path, $classname);
        impl $crate::element::Container for $ty_path {}
    }
}


// An widget is any element that maps to the graphical user interface.
pub trait Widget : Element + Node {
    /// Creates (maps) the native interface objects corresponding to the given IUP interface elements. 
    ///
    /// It will also called recursively to create the native element of all the children in the
    /// element's tree.
    ///
    /// The element must be already attached to a mapped container, except the dialog.
    /// A child can only be mapped if its parent is already mapped.
    ///
    /// This function is automatically called before the dialog is shown in
    ///  `Element::show`, `IupShowXY` (TODO) and `IupPopup` (TODO).
    ///
    /// The function returns success if the element is already mapped and if the native creation
    /// was successful.
    fn map(&mut self) -> Result<(), String> {
        errchk!(unsafe { iup_sys::IupMap(self.raw()) })
    }

    /// Unmap the element from the native system. It will also unmap all its children.
    ///
    /// It will **not** detach the element from its parent, and it will **not** destroy the element.
    fn unmap(&mut self) {
        unsafe { iup_sys::IupUnmap(self.raw()) }
    }

    /// Shows an interfance element.
    ///
    /// Displays a dialog in the current position, or changes a control VISIBLE attribute. If the
    /// dialog needs to be mapped and the current position is not known then the dialog is centered.
    ///
    /// This function can be executed more than once for the same dialog. This will make the dialog
    /// be placed above all other dialogs in the application, changing its Z-order, and update
    /// its position and/or size on screen. 
    fn show(&mut self) -> Result<(), String> {
        errchk!(unsafe { iup_sys::IupShow(self.raw()) })
    }

    /// Hides an interface element.
    ///
    /// This function has the same effect as attributing value "NO" to the interface element’s
    /// VISIBLE attribute.
    fn hide(&mut self) -> Self {
        unsafe { iup_sys::IupHide(self.raw()) };
        self.clone()
    }
}
