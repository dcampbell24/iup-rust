//! Common operations between objects.
//!
//! Every IUP object is so called an element and can be encapsulated in a `Handle`.
use iup_sys;
use libc::{c_void, c_char, c_int};
use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::result::Result;

/// Makes a Vec of `Element` trait objects.
///
/// This actually uses the `Handle` wrapper instead of `Element` due to the Sized requirement.
/// 
/// This should be passed to functions that expect a list of elements in the constructor.
#[macro_export]
macro_rules! elements {
    () => { vec! [] };
    ($($elem:expr),+,) => { elements! [ $($elem),+ ] };
    ($($elem:expr),*) => { vec! [ $($crate::element::Handle::from($elem)),* ] };
}

/// This macro should be used for every type binding IUP handles.
///
/// See applicable `$classname`s [here][1]. Some classes aren't on the list and should be
/// picked up manually by looking at the IUP source code or by looking at the result
/// of `Element::classname`.
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
macro_rules! impl_element {
    ($ty_path:path, $classname:expr) => {
        impl_element_nofrom!($ty_path, $classname);

        impl From<$ty_path> for $crate::element::Handle {
            fn from(elem: $ty_path) -> $crate::element::Handle {
                unsafe { $crate::element::Handle::from_raw_unchecked(elem.raw()) }
            }
        }
    };
}
/// This is called from impl_element! to do all the work.
///
/// This is a necessary thing because if we implemented `From<$ty_path> for Handle` here it'd cause
/// a compilation error during `From<Handle> for Handle`.
macro_rules! impl_element_nofrom {
    ($ty_path:path, $classname:expr) => {

        impl $crate::Element for $ty_path {
            #[inline(always)]
            fn dup(&self) -> Self {
                $ty_path(self.0)
            }
            #[inline(always)]
            fn raw(&self) -> *mut iup_sys::Ihandle {
                self.0
            }
            #[inline(always)]
            unsafe fn from_raw_unchecked(ih: *mut iup_sys::Ihandle) -> Self {
                $ty_path(ih)
            }
            #[inline]
            unsafe fn target_classname() -> &'static str {
                $classname
            }
        }

        impl ::std::fmt::Debug for $ty_path {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                fmt.write_fmt(format_args!("{}({:p})", stringify!($ty_path), self.raw()))
            }
        }

        impl Copy for $ty_path {}
        
        impl Clone for $ty_path {
            fn clone(&self) -> $ty_path {
                *self
            }
        }

        impl $crate::callback::DestroyCb for $ty_path {}
    };
}



/// An object that can wrap **any** IUP element.
pub struct Handle(*mut iup_sys::Ihandle);

impl Handle {

    /// Constructs from a name associated with a element handle (with `Element::add_handle_name` or LED).
    pub fn from_named<S: Into<String>>(name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetHandle(cname.as_ptr()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Converts this handle object into a element object if they are compatible.
    pub fn try_downcast<E: Element>(self) -> Result<E, Handle> {
        if self.can_downcast::<E>() {
            // Since a Handle must be obtained also by using `from_raw` we can assume the handle
            // has reached the Rust binding thought it and thus using `from_raw_unchecked` here.
            Ok(unsafe { E::from_raw_unchecked(self.raw()) })
        } else {
            Err(self)
        }
    }

    /// Checks if this Element type can be downcasted to the type E.
    fn can_downcast<E: Element>(&self) -> bool {
        let lhs = unsafe { self.classname().to_bytes() };
        let rhs = unsafe { E::target_classname().as_bytes() };
        lhs == rhs || rhs == b"__iuprusthandle"
        // In case self/lhs (a Handle) is trying to cast to a target object of Handle, let it go.
    }
}

impl_element_nofrom!(Handle, "__iuprusthandle");
/// Note: The wrapped element may not support `MapCb`.
impl ::callback::MapCb for Handle {}
/// Note: The wrapped element may not support `UnmapCb`.
impl ::callback::UnmapCb for Handle {}
/// Note: The wrapped element may not support `GetFocusCb`.
impl ::callback::GetFocusCb for Handle {}
/// Note: The wrapped element may not support `KillFocusCb`.
impl ::callback::KillFocusCb for Handle {}
/// Note: The wrapped element may not support `EnterWindowCb`.
impl ::callback::EnterWindowCb for Handle {}
/// Note: The wrapped element may not support `LeaveWindowCb`.
impl ::callback::LeaveWindowCb for Handle {}
/// Note: The wrapped element may not support `HelpCb`.
impl ::callback::HelpCb for Handle {}
// TODO impl K_ callbacks when it's implemented.


/// Every IUP object is an `Element`.
pub trait Element where Self: Sized {

    /// Constructs a specialized Element object from a general Handle if they are compatible.
    fn from_handle(handle: Handle) -> Result<Self, Handle> {
        handle.try_downcast::<Self>()
    }

    /// Constructs from a name associated with a element handle (with `Element::add_handle_name` or LED).
    fn from_name<S: Into<String>>(name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetHandle(cname.as_ptr()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Constructs an Element from a raw IUP handle.
    ///
    /// # Safety
    /// The `from_raw_unchecked` method is faster than `from_raw` but must be used with care.
    ///
    /// The Rust binding performs important operations and checks when a raw IUP handle reaches
    /// the bounds of safe Rust binding, that only happens when `from_raw` is used. Be sure the
    /// raw handle has reached safe Rust bounds at least once before using this method.
    ///
    /// It's undefined behaviour if the raw handle is incompatible with `Self` bindings.
    /// Instead use the `Element::from_handle` to perform safe downcasting.
    unsafe fn from_raw_unchecked(ih: *mut iup_sys::Ihandle) -> Self;

    /// Constructs an Element from a raw IUP handle.
    ///
    /// It's undefined behaviour if the raw handle is incompatible with `Self` bindings.
    /// Instead use the `Element::from_handle` to perform safe downcasting.
    ///
    /// # Panics
    /// Panics if the raw handle is a null pointer.
    fn from_raw(ih: *mut iup_sys::Ihandle) -> Self {
        if ih.is_null() {
            panic!("Failed to create IUP element from raw handle because the handle is null.")
        } else {
            unsafe {
                // Note: DESTROY_CB is used here instead of LDESTROY_CB because the DESTROY_CB 
                // is called later. LDESTROY_CB is used in callbacks.rs, see it for more details.
                iup_sys::IupSetCallback(ih, cstr!("DESTROY_CB"), on_element_destroy);
                Element::from_raw_unchecked(ih)
            }
        }
    }

    /// Gets the raw IUP handle associated with this element.
    fn raw(&self) -> *mut iup_sys::Ihandle;

    /// Constructs another object that binds to the same IUP handle as this one.
    #[doc(hidden)]          // This function is now deperecated, Copy+Clone should be used.
    fn dup(&self) -> Self;  // TODO remove this method.

    /// Destroys an interface element and all its children.
    ///
    /// Only dialogs, timers, popup menus and images should be normally destroyed, but **detached**
    /// controls can also be destroyed.
    ///
    /// Menu bars associated with dialogs are automatically destroyed when the dialog is destroyed. 
    ///
    /// Images associated with controls are **NOT** automatically destroyed. The application must
    /// destroy them when they are not used anymore.
    fn destroy(self) {
        unsafe { iup_sys::IupDestroy(self.raw()) };
    }

    /// Sets an interface element attribute.
    ///
    /// See also the [IUP Attributes Guide][1].
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/attrib_guide.html
    fn set_attrib<S1, S2>(&mut self, name: S1, value: S2) -> Self
                                        where S1: Into<String>, S2: Into<String> {
        // The way IupSetAttribute works is infeasible to safety. Use IupSetStrAttribute.
        let cname = CString::new(name.into()).unwrap();
        let cvalue = CString::new(value.into()).unwrap();
        unsafe { iup_sys::IupSetStrAttribute(self.raw(), cname.as_ptr(), cvalue.as_ptr()) };
        self.dup()
    }

    /// Gets an interface element attribute.
    ///
    /// See also the [IUP Attributes Guide][1].
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/attrib_guide.html
    fn attrib<S: Into<String>>(&self, name: S) -> Option<String> {
        // Notice IupGetAttribute does not really give strings but pointers (that may be anything)
        // most (if not all) the default IUP attributes are string values, so we are safe by
        // defaulting to IupGetAttribute. A method should be defined to deal with raw attributes.
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetAttribute(self.raw(), cname.as_ptr()) } {
            cvalue if cvalue.is_null() => None,
            cvalue => Some(string_from_cstr!(cvalue)),
        }
    }

    /// Sets a raw interface element attribute.
    ///
    /// # Safety
    /// While this function is not unsafe, care must be taken while using it, prefer to use
    /// `Element::set_attrib` instead. The `data` pointer must live long enough (most of the time
    /// statically).
    ///
    /// See also the [IUP Attributes Guide][1].
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/attrib_guide.html
    fn set_attrib_data<S1>(&mut self, name: S1, data: *const c_void) -> Self
                                                              where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetAttribute(self.raw(), cname.as_ptr(), data as *const c_char) };
        self.dup()
    }

    /// Gets a raw interface element attribute.
    ///
    /// See also the [IUP Attributes Guide][1].
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/attrib_guide.html
    fn attrib_data<S1>(&mut self, name: S1) -> *mut c_void
                                       where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupGetAttribute(self.raw(), cname.as_ptr()) as *mut c_void }
    }

    /// Associates a element with an attribute.
    ///
    /// Instead of using `Element::add_handle_name` and `Element::set_attrib` with a new creative
    /// name, this function automatically creates a non conflict name and associates the name
    /// with the attribute.
     fn set_attrib_handle<S1, E>(&mut self, name: S1, elem: E) -> Self
                                                where S1: Into<String>, E: Element {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetAttributeHandle(self.raw(), cname.as_ptr(), elem.raw()) };
        self.dup()
    }   

    /// Gets the handle associated with an attribute.
    fn attrib_handle<S1>(&mut self, name: S1) -> Option<Handle>
                                    where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetAttributeHandle(self.raw(), cname.as_ptr()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Clears the value associated with an attribute and use the default value.
    fn clear_attrib<S: Into<String>>(&mut self, name: S) -> Self {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetAttribute(self.raw(), cname.as_ptr(), ptr::null()) };
        self.dup()
    }

    /// Removes an attribute from element and its children if the attrib is inheritable.
    ///
    /// It is useful to reset the state of inheritable attributes in a tree of elements.
    fn reset_attrib<S: Into<String>>(&mut self, name: S) -> Self {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupResetAttribute(self.raw(), cname.as_ptr()) };
        self.dup()
    }

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
        self.dup()
    }

    /// Gets the [class name][1] of this element.
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
    unsafe fn classname(&self) -> &CStr {
        CStr::from_ptr(iup_sys::IupGetClassName(self.raw()))
    }

    /// Gets the [class name][1] the derived object should be targeting.
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
    unsafe fn target_classname() -> &'static str;



    /// Inserts an interface element at the end of the container, after the last element on it.
    ///
    /// This function can be used when elements that will compose a container are not known a *priori*
    /// and should be *dynamically* constructed.
    ///
    /// Valid for any element that contains other elements like dialog, frame, hbox, vbox,
    /// zbox or menu.
    ///
    /// The `new_child` can **not** be mapped. It will **not** map the `new_child` into the native
    /// system. If the parent is already mapped you must explicitly call `Element::map` for the new child.
    ///
    /// The elements are **not** immediately repositioned. Call `Element::refresh` for the container
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
    fn append<E: Element>(&mut self, new_child: E) -> Result<Handle, E>  {
        match unsafe { iup_sys::IupAppend(self.raw(), new_child.raw()) } {
            ptr if ptr.is_null() => Err(new_child),
            ptr => Ok(Handle::from_raw(ptr)),
        }
    }

    /// Detaches an interface element from its parent.
    ///
    /// It will automatically call `Element::unmap` to unmap the element if necessary,
    /// and then detach the element.
    ///
    /// If left detached it is still **necessary to call `Element::destroy`** to destroy the
    /// detached element.
    ///
    /// The elements are **not** immediately repositioned. Call `Element::refresh` for the
    /// container (or any other element in the dialog) to update the dialog layout.
    fn detach(&mut self) -> Self {
        unsafe { iup_sys::IupDetach(self.raw()) };
        self.dup()
    }

    /// Inserts an interface element before another child of the container.
    ///
    /// TODO ref_child NULL doc. See #23.
    ///
    /// See `Element::append` for more details on the semantics of this method.
    fn insert<E1, E2>(&mut self, ref_child: &E1, new_child: E2) -> Result<Handle, E2>
                where E1: Element, E2: Element {
        match unsafe { iup_sys::IupInsert(self.raw(), ref_child.raw(), new_child.raw()) } {
            ptr if ptr.is_null() => Err(new_child),
            ptr => Ok(Handle::from_raw(ptr)),
        }
    }

    /// Moves an interface element from one position in the hierarchy tree to another.
    ///
    /// TODO ref_child NULL doc. See #23.
    ///
    /// See `Element::append` for more details on the semantics of this method.
    fn reparent<E1, E2>(&mut self, new_parent: E1, ref_child: E2) -> Result<(), String>
                where E1: Element, E2: Element {
        errchk!(unsafe { iup_sys::IupReparent(self.raw(), new_parent.raw(), ref_child.raw()) })
    }

    /// Returns the parent of a element.
    fn parent(&self) -> Option<Handle> {
        match unsafe { iup_sys::IupGetParent(self.raw()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
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
    /// See `Element::child` for additional details on the semantics of child positions.
    fn child_pos<E: Element>(&self, child: &E) -> Option<usize> {
        match unsafe { iup_sys::IupGetChildPos(self.raw(), child.raw()) } {
            -1 => None,
            id => Some(id as usize),
        }
    }

    /// Returns the number of children of the given element.
    fn child_count(&self) -> usize {
        unsafe { iup_sys::IupGetChildCount(self.raw()) as usize }
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
    /// layout of only a subset of the dialog use `Element::refresh_children`.
    ///
    /// This function will **not** change the size of the dialog, except if the SIZE or RASTERSIZE
    /// attributes of the dialog where changed before the call. Changing the size of elements
    /// without changing the dialog size may position some controls outside the dialog area at the
    /// left or bottom borders (the elements will be cropped at the dialog borders by the native system).
    ///
    /// `Element::map` also updates the dialog layout, but only when called for the dialog itself,
    /// even if the dialog is already mapped. Since IupShow (TODO), IupShowXY (TODO) and IupPopup (TODO)
    /// call `Element::map`, then they all will always update the dialog layout before showing it,
    /// even also if the dialog is already visible.
    fn refresh(&mut self) {
        unsafe { iup_sys::IupRefresh(self.raw()) };
    }

    /// Updates the size and layout of controls after changing size attributes,
    /// or attributes that affect the size of the control.
    ///
    /// Can be used for any element inside a dialog, only its children will be updated.
    ///
    /// The given element must be a container. It must be inside a dialog hierarchy and must be
    /// mapped. It can not be a dialog. For dialogs use `Element::refresh`.
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

    /// Converts a x,y coordinate in an item position in the container.
    ///
    /// The x,y coordinates are relative to the left corner and top corner of the element.
    ///
    /// This have a different effect for each control it is applied to.
    fn convert_xy_to_pos(&self, x: i32, y: i32) -> Option<i32> {
        match unsafe { iup_sys::IupConvertXYToPos(self.raw(), x as c_int, y as c_int) } {
            -1 => None,
            id => Some(id),
        }
    }


    /// Returns the identifier of an interface element that has an associated handle name using
    /// `Element::add_handle_name` or using LED.
    ///
    /// Handle names shouldn't be confused with the NAME attribute.
    fn handle_name(&self) -> Option<String> {
        match unsafe { iup_sys::IupGetName(self.raw()) } {
            name if name.is_null() => None,
            name => Some(string_from_cstr!(name)),
        }
    }

    /// Associates a handle name with an interface element.
    ///
    /// Can be called several times with the same element and different names.
    /// There is no restriction for the number of names a pointer can have, but `Element::name`
    /// will return the first name found.
    ///
    /// Returns the handle of the interface element previously associated to the parameter name.
    ///
    /// Handle names shouldn't be confused with the NAME attribute.
    fn add_handle_name<S: Into<String>>(&self, name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupSetHandle(cname.as_ptr(), self.raw()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Clears the handle name association on the specified name.
    ///
    /// Note this will not destroy associated elements, just remove a name from the
    /// association table.
    ///
    /// Returns the handle of the interface element previously associated to the parameter name.
    ///
    /// Handle names shouldn't be confused with the NAME attribute.
    fn clear_handle_name<S: Into<String>>(name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupSetHandle(cname.as_ptr(), ptr::null_mut()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }


    //

    fn set_attrib_rgb<S1>(&mut self, name: S1, rgb: (u8, u8, u8)) -> Self
                                                              where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetRGB(self.raw(), cname.as_ptr(), rgb.0, rgb.1, rgb.2) };
        self.dup()
    }
    fn attrib_rgb<S1>(&mut self, name: S1) -> Option<(u8, u8, u8)>
                                       where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        if self.does_attrib_exist(&cname) {
            let mut rgb: (u8, u8, u8) = unsafe { mem::uninitialized() };
            unsafe { iup_sys::IupGetRGB(self.raw(), cname.as_ptr(), &mut rgb.0, &mut rgb.1, &mut rgb.2) };
            Some(rgb)
        } else {
            None
        }
    }
    fn does_attrib_exist(&mut self, cname: &CString) -> bool {
        let attrib = unsafe { iup_sys::IupGetAttribute(self.raw(), cname.as_ptr()) };
        !attrib.is_null()
    }
}


// TODO not exactly sure if this is the place for the global attribute functions

/// Sets an attribute in the global environment.
///
/// If the driver process the attribute then it will not be stored internally.
pub fn set_global<S1, S2>(name: S1, value: S2) 
                                where S1: Into<String>, S2: Into<String> {
    let cname = CString::new(name.into()).unwrap();
    let cvalue = CString::new(value.into()).unwrap();
    unsafe { iup_sys::IupSetStrGlobal(cname.as_ptr(), cvalue.as_ptr()) };
}

/// Returns an attribute value from the global environment.
///
/// The value can be returned from the driver or from the internal storage.
///
/// This function’s return value is not necessarily the same one used by the application to
/// set the attribute’s value.
pub fn global<S: Into<String>>(name: S) -> Option<String> {
    let cname = CString::new(name.into()).unwrap();
    match unsafe { iup_sys::IupGetGlobal(cname.as_ptr()) } {
        cvalue if cvalue.is_null() => None,
        cvalue => Some(string_from_cstr!(cvalue)),
    }
}

/// Clears the value associated with an global attribute.
pub fn clear_attrib<S: Into<String>>(name: S) {
    let cname = CString::new(name.into()).unwrap();
    unsafe { iup_sys::IupSetGlobal(cname.as_ptr(), ptr::null()) };
}

pub fn set_global_data<S1>(name: S1, data: *const c_void)
                                            where S1: Into<String> {
    let cname = CString::new(name.into()).unwrap();
    unsafe { iup_sys::IupSetGlobal(cname.as_ptr(), data as *const _) };
}

pub fn global_data<S1>(name: S1) -> *mut c_void
                           where S1: Into<String> {
    let cname = CString::new(name.into()).unwrap();
    unsafe { iup_sys::IupGetGlobal(cname.as_ptr()) as *mut c_void }
}


/// Called whenever a Element gets destroyed.
///
/// Use this to perform frees related to the Rust binding that are per-element.
extern fn on_element_destroy(ih: *mut iup_sys::Ihandle) -> c_int {
    unsafe { ::callback::drop_callbacks(ih); }
    iup_sys::IUP_DEFAULT
}
