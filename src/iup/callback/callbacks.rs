
// http://sourceforge.net/p/iup/iup/2620/tree//trunk/iup/include/iupcbs.h

// This is the common version of the ACTION callback, any so called ACTION that does not
// have this Fn signature should be another trait.
impl_callback! {
    /// Action generated when the element is activated. Affects each element differently.
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

