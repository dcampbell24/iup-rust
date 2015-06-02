#[macro_use]
extern crate iup;

use iup::Element;
use iup::dialog::Dialog;
use iup::layout::HBox;
use iup::control::{Button, Text};
use iup::callback::Action;

fn main() {
    iup::with_iup(|| {

        let mut text = Text::new()
                        .set_attrib("VALUE", "0")
                        .set_attrib("READONLY", "YES");

        let button = Button::with_title("Count")
                            .set_action(move |_| {
                                let count = text.attrib("VALUE").unwrap().parse::<i32>().unwrap();
                                text.set_attrib("VALUE", (count + 1).to_string());
                            });

        let mut dialog = Dialog::new(
            HBox::new(elements![text, button])
                 .set_attrib("ALIGNMENT", "ACENTER")
                 .set_attrib("MARGIN", "10x10")
                 .set_attrib("GAP", "10")
        );

        dialog.show()

    }).unwrap();
}
