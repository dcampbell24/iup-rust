//! Example based on hello1 from http://wiki.call-cc.org/iup-tutor
#[macro_use]
extern crate iup;

use iup::prelude::*;
use iup::layout::VBox;
use iup::control::{Button, Label};

fn main () {
    iup::with_iup(|| {
        let button = Button::with_title("Ok")
                            .set_attrib("EXPAND", "YES")
                            .set_attrib("TIP", "Exit button")
                            .set_action(|_| CallbackReturn::Close);

        let label = Label::with_title("Hello, world!");

        let vbox = VBox::new(elements![label, button])
                        .set_attrib("GAP", "10")
                        .set_attrib("MARGIN", "10x10")
                        .set_attrib("ALIGNMENT", "ACENTER");

        Dialog::new(vbox)
                .set_attrib("TITLE", "Hello")
                .show()

    }).unwrap();
}
