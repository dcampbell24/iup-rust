//! Example based on hello0 from http://wiki.call-cc.org/iup-tutor

extern crate iup;

fn main() {
    iup::with_iup(|| {
	    let mut dialog = iup::dialog(iup::label("Hello, world!"));
	    iup::show(&mut dialog)
    }).unwrap();
}
