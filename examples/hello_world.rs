extern crate iup;

fn main() {
    iup::open().unwrap();

    let mut dialog = iup::dialog(iup::label("Hello, world!"));
    iup::show(&mut dialog).unwrap();

    iup::main_loop();
    iup::close();
}
