// TODO MOD DOC
use iup_sys;
use std::ptr;
use std::ffi::{CStr, CString};
use callback::CallbackReturn;
use Result;

// TODO the objects could prehaps be copy as they are loose handles?
// it does not implement Clone yet either, so we should pick something.


/// Makes a Vec of `Element` trait objects.
///
/// This actually uses the `Handle` wrapper instead of `Element` due to the Sized requirement.
/// 
/// This should be passed to functions that expect a list of elements in the constructor and such.
#[macro_export]
macro_rules! elements {
    () => { vec! [] };
    ($($elem:expr),*) => { vec! [ $($crate::element::Handle::from_element($elem)),* ] };
}

/// This macro should be used for every type binding IUP handles.
///
/// See applicable `$classname`s [here][1].  Use a empty string if not applicable.
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
macro_rules! impl_element {
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

        use std::fmt;
        impl fmt::Debug for $ty_path {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_fmt(format_args!("{}({:p})", stringify!($ty_path), self.raw()))
            }
        }

        impl $crate::callback::DestroyCb for $ty_path {}
        impl $crate::callback::MapCb for $ty_path {}
        impl $crate::callback::UnmapCb for $ty_path {}
        impl $crate::callback::GetFocusCb for $ty_path {}
        impl $crate::callback::KillFocusCb for $ty_path {}
        impl $crate::callback::EnterWindowCb for $ty_path {}
        impl $crate::callback::LeaveWindowCb for $ty_path {}
        impl $crate::callback::HelpCb for $ty_path {}
    };
}



/// An object that can wrap **any** IUP element/handle.
pub struct Handle(*mut iup_sys::Ihandle);

impl Handle {
    /// Constructs from another elementar object.
    pub fn from_element<E: Element>(elem: E) -> Handle {
        Handle(elem.raw())
    }

    /// Converts this handle object into a element object if they are compatible.
    pub fn to_element<E: Element>(self) -> Option<E> {
        if self.can_downcast::<E>() {
            // Since a Handle must be obtained also by using `from_raw` we can assume the handle
            // has reached the Rust binding thought it and thus using `from_raw_unchecked` here.
            Some(unsafe { E::from_raw_unchecked(self.raw()) })
        } else {
            None
        }
    }

    /// Checks if this Element type can be downcasted to the type E.
    fn can_downcast<E: Element>(&self) -> bool {
        // TODO what about filedlg, colordlg and such that are essentially "dialog"?
        let lhs = unsafe { self.classname().to_bytes() };
        let rhs = unsafe { E::target_classname().as_bytes() };
        if lhs.len() > 0 && rhs.len() > 0 {
            lhs == rhs
        } else {
            // In case self (a Handle) is trying to cast to a target object of Handle, let it go.
            rhs == b"__iuprusthandle"
        }
    }
}

impl_element!(Handle, "__iuprusthandle");


/// Every IUP object is an `Element`.
pub trait Element where Self: Sized {

    /// Gets the raw IUP handle associated with this element.
    fn raw(&self) -> *mut iup_sys::Ihandle;

    /// Constructs another object that binds to the same IUP handle as this one.
    fn dup(&self) -> Self;

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
    unsafe fn from_raw_unchecked(ih: *mut iup_sys::Ihandle) -> Self;

    /// Constructs an Element from a raw IUP handle.
    ///
    /// It's undefined behaviour if the raw handle is incompatible with `Self` bindings.
    ///
    /// # Panics
    /// Panics if the raw handle is a null pointer.
    fn from_raw(ih: *mut iup_sys::Ihandle) -> Self {
        if ih.is_null() {
            panic!("Failed to create IUP Widget from raw handle.")
        } else {
            unsafe {
                // Note: DESTROY_CB is used here instead of LDESTROY_CB because the DESTROY_CB 
                // is called later. LDESTROY_CB is used in callback.rs, see it for more details.
                iup_sys::IupSetCallback(ih, str_to_c_str!("DESTROY_CB"), on_element_destroy);
                Element::from_raw_unchecked(ih)
            }
        }
    }

    /// Sets an interface element attribute.
    ///
    /// See also the [IUP Attributes Guide][1].
    ///
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
    ///
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/attrib_guide.html
    fn attrib<S>(&self, name: S) -> Option<String>
                                  where S: Into<String> {
        // Notice IupGetAttribute does not really give strings but pointers (that may be anything)
        // most (if not all) the default IUP attributes are string values, so we are safe by
        // defaulting to IupGetAttribute. A method should be defined to deal with raw attributes.
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetAttribute(self.raw(), cname.as_ptr()) } {
            cvalue if cvalue.is_null() => None,
            cvalue => Some(string_from_c_str!(cvalue)),
        }
    }

    /// Clears the value associated with an attribute and use the default value.
    fn clear_attrib<S>(&mut self, name: S) where S: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetAttribute(self.raw(), cname.as_ptr(), ptr::null()) };
    }

    /// Removes an attribute from element and its children if the attrib is inheritable.
    ///
    /// It is useful to reset the state of inheritable attributes in a tree of elements.
    fn reset_attrib<S>(&mut self, name: S) where S: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupResetAttribute(self.raw(), cname.as_ptr()) };
    }

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
    fn map(&mut self) -> Result<()> {
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
    fn show(&mut self) -> Result<()> {
        errchk!(unsafe { iup_sys::IupShow(self.raw()) })
    }

    /// Hides an interface element.
    ///
    /// This function has the same effect as attributing value "NO" to the interface element’s
    /// VISIBLE attribute.
    fn hide(&mut self) {
        unsafe { iup_sys::IupHide(self.raw()) };
    }

    /// Gets the [class name][1] of this element.
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
    unsafe fn classname(&self) -> &CStr {
        CStr::from_ptr(iup_sys::IupGetClassName(self.raw()))
    }

    /// Gets the [class name][1] the derived object should be targeting.
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
    unsafe fn target_classname() -> &'static str;



    // TODO
    // class
    // native handle
    // expand
    // x,y
    // userwidth, userheight
    // naturalwidth, naturalheight
    // currentwidth, currentheight
    // parent
    // first child
    // brother
}

/// Called whenever a Element gets destroyed.
///
/// Use this to perform frees related to the Rust binding that are per-element.
extern fn on_element_destroy(ih: *mut iup_sys::Ihandle) -> iup_sys::CallbackReturn {
    unsafe { ::callback::drop_callbacks(ih); }
    iup_sys::CallbackReturn::Default
}
