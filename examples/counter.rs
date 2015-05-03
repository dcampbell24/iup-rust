extern crate iup;

use iup::CallbackReturn;

extern fn button_1_cb(_ih: *mut iup::IhandleRaw) -> CallbackReturn {
    let mut text = iup::get_handle("text").unwrap();
    let count = iup::get_attribute(&mut text, "VALUE").unwrap().parse::<i32>().unwrap();
    iup::set_str_attribute(&mut text, "VALUE", &(count + 1).to_string());
    CallbackReturn::Default
}

extern fn button_2_cb(_ih: *mut iup::IhandleRaw) -> CallbackReturn {
    let mut text = iup::get_handle("text").unwrap();
    let count = iup::get_attribute(&mut text, "VALUE").unwrap().parse::<i32>().unwrap();
    iup::set_str_attribute(&mut text, "VALUE", &(count - 1).to_string());
    CallbackReturn::Default
}

fn main() {
    iup::open().unwrap();

    let label = iup::label("Counter");

    let mut text = iup::text();
    iup::set_handle("text", &mut text);
    iup::set_str_attribute(&mut text, "READONLY", "YES");
    iup::set_str_attribute(&mut text, "VALUE", "0");

    let mut button_1 = iup::button("Count Up");
    iup::set_callback(&mut button_1, "ACTION", button_1_cb);

    let mut button_2 = iup::button("Count Down");
    iup::set_callback(&mut button_2, "ACTION", button_2_cb);

    let mut hbox = iup::hboxv(vec!(label, text, button_1, button_2));
    iup::set_str_attribute(&mut hbox, "ALIGNMENT", "ACENTER");

    iup::show(&mut iup::dialog(hbox)).unwrap();

    iup::main_loop();
    iup::close();
}
