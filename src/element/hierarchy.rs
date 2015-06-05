//! Hierarchy operations on elements.
use iup_sys;
use libc::c_int;
use std::ffi::CString;
use std::result::Result;

use element::{Element, Handle};

/// Containers are elements that can store childs.
pub trait Container : Node {

    /// Inserts an interface element at the end of the container, after the last element on it.
    ///
    /// This function can be used when elements that will compose a container are not known a *priori*
    /// and should be *dynamically* constructed.
    ///
    /// Valid for any element that contains other elements like dialog, frame, hbox, vbox,
    /// zbox or menu.
    ///
    /// The `new_child` can **not** be mapped. It will **not** map the `new_child` into the native
    /// system. If the parent is already mapped you must explicitly call `Widget::map` for the new child.
    ///
    /// The elements are **not** immediately repositioned. Call `Node::refresh` for the container
    /// (or any other element in the dialog) to update the dialog layout.
    ///
    /// If the actual parent is a layout box (`VBox`, `HBox` or `ZBox`) and you try to append a
    /// child that it is already at the parent child list, then the child is moved to the last
    /// child position.
    ///
    /// Returns the actual parent if the interface element was successfully inserted. Otherwise
    /// returns the desired `new_child`. Failed can happen for instance if this element is not
    /// a container for other elements or the `new_child` is already a child (except on layout boxes).
    /// Notice that the desired parent can contains a set of elements and containers where the
    /// child will be actually attached so the function returns the actual parent of the element.
    fn append<E: Node>(&mut self, new_child: E) -> Result<Handle, E>  {
        match unsafe { iup_sys::IupAppend(self.raw(), new_child.raw()) } {
            ptr if ptr.is_null() => Err(new_child),
            ptr => Ok(Handle::from_raw(ptr)),
        }
    }

    /// Inserts an interface element before another child of the container.
    ///
    /// TODO ref_child NULL doc. See #23.
    ///
    /// See `Container::append` for more details on the semantics of this method.
    fn insert<E1, E2>(&mut self, ref_child: &E1, new_child: E2) -> Result<Handle, E2>
                    where E1: Node, E2: Node {
        match unsafe { iup_sys::IupInsert(self.raw(), ref_child.raw(), new_child.raw()) } {
            ptr if ptr.is_null() => Err(new_child),
            ptr => Ok(Handle::from_raw(ptr)),
        }
    }

    /// Returns the a child of the element given its position.
    ///
    /// The position `pos` starts from 0.
    ///
    /// This function will return the children of the element in the exact same order in
    /// which they were assigned.
    fn child(&self, pos: usize) -> Option<Handle> {
        match unsafe { iup_sys::IupGetChild(self.raw(), pos as c_int) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Returns the position of a child of the given control. 
    ///
    /// See `Container::child` for additional details on the semantics of child positions.
    fn child_pos<E: Node>(&self, child: &E) -> Option<usize> {
        match unsafe { iup_sys::IupGetChildPos(self.raw(), child.raw()) } {
            -1 => None,
            id => Some(id as usize),
        }
    }

    /// Returns the number of children of the given element.
    fn child_count(&self) -> usize {
        unsafe { iup_sys::IupGetChildCount(self.raw()) as usize }
    }
}

/// Nodes are elements that can be part of a hierarchical structure.
pub trait Node : Element {

    /// Detaches an interface element from its parent.
    ///
    /// It will automatically call `Widget::unmap` to unmap the element if necessary,
    /// and then detach the element.
    ///
    /// If left detached it is still **necessary to call `Element::destroy`** to destroy the
    /// detached element.
    ///
    /// The elements are **not** immediately repositioned. Call `Node::refresh` for the
    /// container (or any other element in the dialog) to update the dialog layout.
    fn detach(&mut self) -> Self {
        unsafe { iup_sys::IupDetach(self.raw()) };
        self.clone()
    }

    /// Moves an interface element from one position in the hierarchy tree to another.
    ///
    /// TODO ref_child NULL doc. See #23.
    ///
    /// See `Container::append` for more details on the semantics of this method.
    fn reparent<E1, E2>(&mut self, new_parent: E1, ref_child: E2) -> Result<Self, Self>
                where E1: Container, E2: Node {
        match unsafe { iup_sys::IupReparent(self.raw(), new_parent.raw(), ref_child.raw()) } {
            iup_sys::IUP_NOERROR => Ok(*self),
            iup_sys::IUP_ERROR => Err(*self),
            _ => unreachable!(),
        }
    }

    /// Returns the parent of a element.
    fn parent(&self) -> Option<Handle> {
        match unsafe { iup_sys::IupGetParent(self.raw()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Returns the brother of an element.
    fn brother(&self) -> Option<Handle> {
         match unsafe { iup_sys::IupGetBrother(self.raw()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Returns the handle of the dialog that contains that interface element.
    ///
    /// Works also for children of a menu that is associated with a dialog.
    fn dialog(&self) -> Option<Handle> {
         match unsafe { iup_sys::IupGetDialog(self.raw()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Returns the identifier of the child element that has the NAME attribute equals to the
    /// given value on the same dialog hierarchy.
    ///
    /// Works also for children of a menu that is associated with a dialog.
    ///
    /// This function will only found the child if the **NAME attribute** is set at the control.
    ///
    /// The function returns immediatelly with the result (not needing to traverse the hierarchy)
    /// after the child is mapped.
    fn dialog_child<S: Into<String>>(&self, name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
         match unsafe { iup_sys::IupGetDialogChild(self.raw(), cname.as_ptr()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Updates the size and layout of all controls in the same dialog. 
    ///
    /// Can be called even if the dialog is not mapped.
    /// Can be used for any control, but it will always affect the whole dialog, to refresh the
    /// layout of only a subset of the dialog use `Node::refresh_children`.
    ///
    /// This function will **not** change the size of the dialog, except if the SIZE or RASTERSIZE
    /// attributes of the dialog where changed before the call. Changing the size of elements
    /// without changing the dialog size may position some controls outside the dialog area at the
    /// left or bottom borders (the elements will be cropped at the dialog borders by the native system).
    ///
    /// `Widget::map` also updates the dialog layout, but only when called for the dialog itself,
    /// even if the dialog is already mapped. Since `Widget::show`, `DialogElement::showxy` and
    /// `DialogElement::popup` call `Widget::map`, then they all will always update the dialog
    /// layout before showing it, even also if the dialog is already visible.
    fn refresh(&mut self) {
        unsafe { iup_sys::IupRefresh(self.raw()) };
    }

    /// Updates the size and layout of controls after changing size attributes,
    /// or attributes that affect the size of the control.
    ///
    /// Can be used for any element inside a dialog, only its children will be updated.
    ///
    /// The given element must be a container. It must be inside a dialog hierarchy and must be
    /// mapped. It can not be a dialog. For dialogs use `Node::refresh`.
    ///
    /// This function will **not** change the size of the given element, even if the natural size of
    /// its children would increase its natural size.
    fn refresh_children(&mut self) { // XXX container specific, maybe move to a container trait
        unsafe { iup_sys::IupRefreshChildren(self.raw()) };
    }

    /// Mark the element to be redraw when the control returns to the system.
    fn update(&self) {
        unsafe { iup_sys::IupUpdate(self.raw()) };
    }

    /// Mark the element children to be redraw when the control returns to the system.
    fn update_children(&self) {
        unsafe { iup_sys::IupUpdateChildren(self.raw()) };
    }

    /// Force the element and its children to be redraw immediately.
    fn redraw(&self, also_redraw_children: bool) {
        unsafe { iup_sys::IupRedraw(self.raw(), also_redraw_children as c_int) };
    }
}
