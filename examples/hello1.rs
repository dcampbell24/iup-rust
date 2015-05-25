//! Example based on hello1 from http://wiki.call-cc.org/iup-tutor
extern crate iup;

use iup::CallbackReturn;

fn main () {
    iup::with_iup(|| {
        let mut btn = iup::button("Ok");
        iup::callback::set_action(&mut btn, Some(|_| CallbackReturn::Close));
        iup::set_str_attribute(&mut btn, "EXPAND", "Yes");
        iup::set_str_attribute(&mut btn, "TIP", "Exit button");
      
        let lbl = iup::label("Hello, world!");

        let mut vb = iup::vboxv(&[lbl, btn]);
        iup::set_str_attribute(&mut vb, "GAP", "10");
        iup::set_str_attribute(&mut vb, "MARGIN", "10x10");
        iup::set_str_attribute(&mut vb, "ALIGNMENT", "ACENTER");

        let mut dlg = iup::dialog(vb);
        iup::set_str_attribute(&mut dlg, "TITLE", "Hello");

        iup::show(&mut dlg)

    }).unwrap();
}
