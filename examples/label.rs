#[macro_use]
extern crate iup;

use iup::prelude::*;
use iup::layout::VBox;
use iup::control::Label;

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
         .set_attrib("RESIZE", "NO")
         .show()

    }).unwrap();
}
