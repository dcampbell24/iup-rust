//! Example based on hello0 from http://wiki.call-cc.org/iup-tutor

extern crate iup;

fn main() {
    iup::open().unwrap();
    let mut dialog = iup::dialog(iup::label("Hello, world!"));
    iup::show(&mut dialog).unwrap();
    iup::main_loop();
    iup::close();
}
