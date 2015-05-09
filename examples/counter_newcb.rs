extern crate iup;

use iup::CallbackReturn;

fn add_to_count(i: i32) {
    let mut text = iup::get_handle("text").unwrap();
    let count = iup::get_attribute(&mut text, "VALUE").unwrap().parse::<i32>().unwrap();
    iup::set_str_attribute(&mut text, "VALUE", &(count + i).to_string());
}

fn main() {
    iup::open().unwrap();

    let label = iup::label("Counter");

    let mut text = iup::text();
    iup::set_handle("text", &mut text);
    iup::set_str_attribute(&mut text, "READONLY", "YES");
    iup::set_str_attribute(&mut text, "VALUE", "0");

    let mut button_1 = iup::button("Count Up");
    iup::callback::set_action(&mut button_1, Some(|_ih| {
        add_to_count(1);
        CallbackReturn::Default
    }));

    let mut button_2 = iup::button("Count Down");
    iup::callback::set_action(&mut button_2, Some(|_ih| {
        add_to_count(-1);
        CallbackReturn::Default
    }));
    iup::callback::set_destroy_cb(&mut button_2, Some(|_ih|{
        println!("Button2 getting destroyed!!");
    }));

    let mut hbox = iup::hboxv(vec!(label, text, button_1, button_2));
    iup::set_str_attribute(&mut hbox, "ALIGNMENT", "ACENTER");

    iup::show(&mut iup::dialog(hbox)).unwrap();

    iup::main_loop();
    iup::close();
}
