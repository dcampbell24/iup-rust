extern crate iup;

fn main() {
    iup::open().unwrap();

    let label = iup::label("Counter");
    let text = iup::text();
    let button_1 = iup::button("Count Up");
    let button_2 = iup::button("Count Down");

    let hbox = iup::hbox(vec!(label, text, button_1, button_2));

    iup::show(&mut iup::dialog(hbox));


    iup::main_loop();
    iup::close();
}
