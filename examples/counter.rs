extern crate iup;

use iup::CallbackReturn;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    iup::open().unwrap();

    let text = Rc::new(RefCell::new(iup::text()));
    {
        let mut text = text.borrow_mut();
        iup::set_handle("text", &mut text);
        iup::set_str_attribute(&mut text, "READONLY", "YES");
        iup::set_str_attribute(&mut text, "VALUE", "0");
    }

    let mut button = iup::button("Count");
    {
        let text = text.clone();
        iup::callback::set_action(&mut button, Some(move |_ih| {
            let mut text = text.borrow_mut();
            let count = iup::get_attribute(&mut text, "VALUE").unwrap().parse::<i32>().unwrap();
            iup::set_str_attribute(&mut text, "VALUE", &(count + 1).to_string());
            CallbackReturn::Default
        }));
        iup::callback::set_destroy_cb(&mut button, Some(|_ih|{
            println!("One of the buttons is getting destroyed!!");
        }));
    }

    let mut hbox = iup::hboxv(vec!(text.borrow().clone(), button));
    iup::set_str_attribute(&mut hbox, "ALIGNMENT", "ACENTER");
    iup::set_str_attribute(&mut hbox, "MARGIN", "10x10");
    iup::set_str_attribute(&mut hbox, "GAP", "10");

    iup::show(&mut iup::dialog(hbox)).unwrap();

    iup::main_loop();
    iup::close();
}
