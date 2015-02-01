extern crate iup;

fn main() {
    iup::open();
    iup::show(iup::dialog(iup::label("Hello, world!")));
    iup::main_loop();
    iup::close();
}
