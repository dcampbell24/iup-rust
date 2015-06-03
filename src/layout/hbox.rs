use iup_sys;

use Handle;
use Element;

/// A void container for composing elements horizontally.
/// It is a box that arranges the elements it contains from left to right.
///
/// See the [IUP HBox Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iuphbox.html
pub struct HBox(*mut iup_sys::Ihandle);

impl HBox {
    /// Creates a horizontal container box with the specified childs.
    pub fn new<A>(elems: A) -> HBox where A: AsRef<[Handle]>  {
        let mut carray = slice_to_ih_array!(elems.as_ref());
        unsafe { HBox::from_raw(iup_sys::IupHboxv(carray.as_mut_ptr())) }
    }
}

impl_widget_container!(HBox, "hbox");
