//! Example based on hello1 from http://wiki.call-cc.org/iup-tutor

extern crate iup;

use iup::CallbackReturn;

extern fn exit_cb(_ih: *mut iup::IhandleRaw) -> CallbackReturn {
    CallbackReturn::Close
}

fn main () {
    iup::open().unwrap();

    let mut btn = iup::button("Ok");
    iup::set_callback(&mut btn, "ACTION", exit_cb);
    iup::set_str_attribute(&mut btn, "EXPAND", "Yes");
    iup::set_str_attribute(&mut btn, "TIP", "Exit button");
  
    let lbl = iup::label("Hello, world!");

    let mut vb = iup::vboxv(vec!(lbl, btn));
    iup::set_str_attribute(&mut vb, "GAP", "10");
    iup::set_str_attribute(&mut vb, "MARGIN", "10x10");
    iup::set_str_attribute(&mut vb, "ALIGNMENT", "ACENTER");

    let mut dlg = iup::dialog(vb);
    iup::set_str_attribute(&mut dlg, "TITLE", "Hello");

    iup::show(&mut dlg).unwrap();

    iup::main_loop();

    iup::destroy(dlg);
    iup::close();
}
