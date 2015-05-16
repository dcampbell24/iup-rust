extern crate iup;
use std::rc::Rc;
use std::cell::RefCell;

use iup::CallbackReturn;

fn main() {
    iup::open().unwrap();

    let label = iup::label("Counter");

    let text = Rc::new(RefCell::new(iup::text()));

    {
        let mut text = text.borrow_mut();
        iup::set_handle("text", &mut text);
        iup::set_str_attribute(&mut text, "READONLY", "YES");
        iup::set_str_attribute(&mut text, "VALUE", "0");
    }

    let mut button_1 = iup::button("Count Up");
    {
        let text = text.clone();
        iup::callback::set_action(&mut button_1, Some(move |_ih| {
            let mut text = text.borrow_mut();
            let count = iup::get_attribute(&mut text, "VALUE").unwrap().parse::<i32>().unwrap();
            iup::set_str_attribute(&mut text, "VALUE", &(count + 1).to_string());
            CallbackReturn::Default
        }));
    }

    let mut button_2 = iup::button("Count Down");
    {
        let text = text.clone();
        iup::callback::set_action(&mut button_2, Some(move |_ih| {
            let mut text = text.borrow_mut();
            let count = iup::get_attribute(&mut text, "VALUE").unwrap().parse::<i32>().unwrap();
            iup::set_str_attribute(&mut text, "VALUE", &(count - 1).to_string());
            CallbackReturn::Default
        }));
        iup::callback::set_destroy_cb(&mut button_2, Some(|_ih|{
            println!("One of the buttons is getting destroyed!!");
        }));
    }

    let mut hbox = iup::hboxv(vec!(label, text.borrow().clone(), button_1, button_2));
    iup::set_str_attribute(&mut hbox, "ALIGNMENT", "ACENTER");

    iup::show(&mut iup::dialog(hbox)).unwrap();

    iup::main_loop();
    iup::close();
}
