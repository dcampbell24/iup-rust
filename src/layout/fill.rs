use iup_sys;

use Element;

/// A void element, which dynamically occupies empty spaces always trying to expand itself.
///
/// Its parent should be an `HBox`, an `VBox` or a `GridBox`, or else this type of expansion
/// will not work. If an EXPAND is set on at least one of the other children of the box,
/// then the fill expansion is ignored.
///
/// See the [IUP Fill Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupfill.html
pub struct Fill(*mut iup_sys::Ihandle);

impl Fill {
    pub fn new() -> Fill {
        unsafe { Fill::from_raw(iup_sys::IupFill()) }
    }
}

impl_widget!(Fill, "fill");
