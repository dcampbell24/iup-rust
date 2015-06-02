#[macro_use]
extern crate iup;

use iup::{Element, Orientation};
use iup::dialog::Dialog;
use iup::layout::{HBox, VBox};
use iup::control::{Button, Label};
use iup::callback::{Action, CallbackReturn};

fn main () {
    iup::with_iup(|| {
        let phrase = "The quick brown fox jumps over the lazy dog";
        Dialog::new(
            VBox::new(elements![

                Label::with_title(phrase),
                Label::new_separator(Orientation::Horizontal),

                Label::with_title(phrase)
                    .set_attrib("FONT", "COURIER_NORMAL_14"),
                Label::new_separator(Orientation::Horizontal),

                VBox::new(elements![
                    Label::with_title(phrase),
                    Label::with_title(phrase),
                ]).set_attrib("FONT", "COURIER_ITALIC_14"),

            ]).set_attrib("GAP", "5x")
              .set_attrib("MARGIN", "10x10")

        ).set_attrib("TITLE", "Label")
         .show()

    }).unwrap();
}
