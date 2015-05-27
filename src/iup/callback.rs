// TODO CRATE DOC

// http://sourceforge.net/p/iup/iup/2620/tree//trunk/iup/include/iupcbs.h

use iup_sys;
use Element;
use CallbackReturn;

macro_rules! fbox_c_str {
    ($cb_name:expr) => {
        // It's important to use the prefix '_IUP*', it's reserved by IUP for internal use and bindings.
        // So we use '_IUPRUST_*' prefix to refer to data reserved for the Rust binding.
        str_to_c_str!(concat!("_IUPRUST_FBOX_", $cb_name))
    }
}

/// Implements a callback binding between C IUP and Rust which accepts closures.
///
/// After this macro is executed the trait `$trait_name` is implemented with the following
/// default methods:
////
///   + `$set_method` to set associate a closure with the callback `$cb_name`.
///   + `$remove_method` to remove a previosly associated callback `$cb_name`.
///   + `$listener_method` which is a high-level receiver of the event and should propagate to
///     it's boxed closure argument. This method is define to have a higher flexibility of what
///     the Fn type of a callback could be. For instance it may be able to not return anything
///     and let the listener method return `CallbackReturn::Default`.
///   + `__ciup_listener` should not be called nor assumed to exist! It is the native C callback
///     that will forward the call to `$listener_method`.
///
/// The signature of the closure is defined by the `F` constraint in `$set_method` which is
/// captured by the macro.
///
/// **Note**: Don't forget to add a dropper for the event in `drop_callbacks` after using this
/// macro. You **must** do so to free allocations associated with closures.
///
macro_rules! impl_callback {

    (
        $(#[$trait_attr:meta])* // allow doc comments here
        pub trait $trait_name:ident where Self: Element {
            let name = $cb_name:expr;

            fn $set_method:ident<F: 'static + Fn(Self $(, $fn_arg_ty:ty),*) -> $fn_ret_ty:ty>(&mut self, cb: F) -> Self;

            fn $remove_method:ident(&mut self) -> Option<Box<_>>;

            fn $listener_method:ident($listener_self:ident, $ls_fbox_arg:ident: &Box<_> $(, $ls_arg:ident: $ls_arg_ty:ty)*)
                -> CallbackReturn
            $listener_method_body:expr
        }
        
    ) => {

        $(#[$trait_attr])*
        pub trait $trait_name where Self: Element {

            fn $set_method<F>(&mut self, cb: F) -> Self
                    where F: 'static + Fn(Self, $($fn_arg_ty),*) -> $fn_ret_ty {

                use std::mem::transmute;

                // TODO remove this in favour to std::boxed::into_raw when it gets stable.
                unsafe fn box_into_raw<T : ?Sized>(b: Box<T>) -> *mut T {
                    transmute(b)
                }

                self.$remove_method();

                unsafe {
                    let fb: Box<Box<Fn(Self, $($fn_arg_ty),*) -> $fn_ret_ty + 'static>> = Box::new(Box::new(cb));
                    iup_sys::IupSetAttribute(self.raw(), fbox_c_str!($cb_name), box_into_raw(fb) as *const _);
                    iup_sys::IupSetCallback(self.raw(), str_to_c_str!($cb_name), transmute(<Self as $trait_name>::__ciup_listener));
                }

                self.dup()
            }

            fn $remove_method(&mut self)
                                -> Option<Box<Fn(Self, $($fn_arg_ty),*) -> $fn_ret_ty + 'static>> {
                unsafe {
                    use std::mem::transmute;
                    use std::ptr;

                    let capsule_box = iup_sys::IupGetAttribute(self.raw(), fbox_c_str!($cb_name))
                                                as *mut Box<Fn(Self, $($fn_arg_ty),*) -> $fn_ret_ty + 'static>;
                    if capsule_box.is_null() {
                        None 
                    } else {

                        // TODO when Box::from_raw gets stable use it instead of transmute here.
                        let inner_box: Box<Box<Fn(Self, $($fn_arg_ty),*) -> $fn_ret_ty + 'static>> = transmute(capsule_box);

                        iup_sys::IupSetAttribute(self.raw(), fbox_c_str!($cb_name), ptr::null());
                        iup_sys::IupSetCallback(self.raw(), str_to_c_str!($cb_name), transmute(ptr::null::<u8>()));

                        Some(*inner_box)
                        // inner_box itself gets freed now
                    }
                }
            }

            fn $listener_method($listener_self, $ls_fbox_arg: &Box<Fn(Self $($fn_arg_ty),*) -> $fn_ret_ty>,
                             $($ls_arg: $ls_arg_ty),*)
                            -> CallbackReturn {
                $listener_method_body
            }

            extern fn __ciup_listener(ih: *mut iup_sys::Ihandle, $($ls_arg: $ls_arg_ty),*)
                    -> CallbackReturn {
                let fbox_ptr  = unsafe {
                                   iup_sys::IupGetAttribute(ih, fbox_c_str!($cb_name))
                                     as *mut Box<Fn(Self, $($fn_arg_ty),*) -> $fn_ret_ty + 'static>
                                };
                assert!(fbox_ptr.is_null() == false);
                let fbox: &Box<_> = unsafe { &(*(fbox_ptr)) };
                let element = unsafe { <Self as Element>::from_raw_unchecked(ih) };
                element.$listener_method(fbox, $($ls_arg: $ls_arg_ty),*)
             }
        }
    }
}

/// Drops the closure associated with the `$cb_name` (literal) callback in the element `$ih`.
///
/// This is a **very hacky** method to free boxed closures, it takes advantage of the layout
/// of the dynamic dispatching of TraitObject to the destructor and also the fact our closures
/// are 'static (thus `Any`).
///
/// For this very reason this may not work on future versions of Rust since the language provides
/// no binary-compatibility guarantances between versions.
/// 
/// It was implemented this way to avoid [too much] extra work for freeing each closure, but as
/// soon as the library gets more mature it's recommended to find a replacement for this method.
macro_rules! drop_callback {
    ($ih:ident, $cb_name:expr) => {{
        use std::mem::transmute;
        use std::any::Any;
        let capsule_box = iup_sys::IupGetAttribute($ih, fbox_c_str!($cb_name))
                                                    as *mut Box<Any>;   // HACK HACK HACK!!!!
        if !capsule_box.is_null() {
            // TODO when Box::from_raw gets stable use it instead of transmute here.
            let inner_box: Box<Box<Any>> = transmute(capsule_box);
            drop(inner_box);
        }
    }}
}

/// TODO how to make this exposed only at crate-level?
///
/// This is called right when a IUP element is being destroyed and it should free up all data
/// associated with callbacks. Just use the `drop_callback!` macro for each callback implemented.
/// 
pub unsafe fn drop_callbacks(ih: *mut iup_sys::Ihandle) {
    drop_callback!(ih, "ACTION");
    drop_callback!(ih, "LDESTROY_CB");
}


// This is the common version of the ACTION callback, any so called ACTION that does not
// have this Fn signature should be another trait.
impl_callback! {
    /// Action generated when the element is activated. Affects each element differently.
    pub trait Action where Self: Element {
        let name = "ACTION";

        fn set_action<F: 'static + Fn(Self) -> CallbackReturn>(&mut self, cb: F) -> Self;
        fn remove_action(&mut self) -> Option<Box<_>>;

        fn listener(self, f: &Box<_>) -> CallbackReturn {
            f(self)
        }
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

        fn set_destroy_cb<F: 'static + Fn(Self) -> ()>(&mut self, cb: F) -> Self;
        fn remove_destroy_cb(&mut self) -> Option<Box<_>>;

        fn listener(self, f: &Box<_>) -> CallbackReturn {
            f(self);
            CallbackReturn::Default
        }
    }
}
