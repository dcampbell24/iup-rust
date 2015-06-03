use iup_sys;

use Handle;
use Element;

/// A void container for composing elements vertically.
/// It is a box that arranges the elements it contains from top to bottom.
///
/// See the [IUP VBox Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupvbox.html
pub struct VBox(*mut iup_sys::Ihandle);

impl VBox {
    /// Creates a vertical container box with the specified childs.
    pub fn new<A>(elems: A) -> VBox where A: AsRef<[Handle]>  {
        let mut carray = slice_to_ih_array!(elems.as_ref());
        unsafe { VBox::from_raw(iup_sys::IupVboxv(carray.as_mut_ptr())) }
    }
}

impl_widget_container!(VBox, "vbox");
