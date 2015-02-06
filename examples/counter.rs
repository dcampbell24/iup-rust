extern crate iup;

fn main() {
    iup::open().unwrap();

    let label = iup::label("Counter");

    let mut text = iup::text();
    iup::set_str_attribute(&mut text, "READONLY", "YES");

    let button_1 = iup::button("Count Up");
    let button_2 = iup::button("Count Down");

    let mut hbox = iup::hboxv(vec!(label, text, button_1, button_2));
    iup::set_str_attribute(&mut hbox, "ALIGNMENT", "ACENTER");

    iup::show(&mut iup::dialog(hbox)).unwrap();


    iup::main_loop();
    iup::close();
}
