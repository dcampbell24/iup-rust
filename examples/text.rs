//! Creates a bunch of useless text controls.

#[macro_use]
extern crate iup;

use iup::prelude::*;
use iup::layout::{HBox, VBox};
use iup::control::Text;

fn main () {
    iup::with_iup(|| {

        let mut info = Text::new()
                            .set_attrib("READONLY", "YES")
                            .set_attrib("EXPAND", "HORIZONTAL")
                            .set_attrib("VALUE", "You can read, but you can't edit.");

        Dialog::new(
            VBox::new(elements![

                info,
                Text::new()
                    .set_attrib("MULTILINE", "YES")
                    .set_attrib("EXPAND", "YES")
                    .set_caret_cb(move |(_, lin, col, pos)| {
                        info.set_attrib("VALUE", format!("Text changed at {}:{}, {}", lin, col, pos));
                    }),
                    
                HBox::new(elements![
                    Text::new_spin()
                            .set_spin_cb(move |(_, pos)| {
                                info.set_attrib("VALUE", format!("Spin changed to '{}'", pos));
                            }),
                    Text::new().set_attrib("PASSWORD", "YES")
                               .set_attrib("VALUE", "123456789")
                               .set_attrib("EXPAND", "HORIZONTAL"),
                ])
            ]).set_attrib("EXPAND", "YES")

        ).set_attrib("TITLE", "Text")
         .set_attrib("SIZE", "200x200")
         .show()

    }).unwrap();
}
