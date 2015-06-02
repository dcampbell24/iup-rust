//! Example based on http://sourceforge.net/p/iup/iup/HEAD/tree/trunk/iup/html/examples/Lua/button.lua
//!
//! Creates four buttons. The first uses images, the second turns the first on and off, the third
//! exits the application and the last does nothing
//!

#[macro_use]
extern crate iup;

use iup::Element;
use iup::dialog::Dialog;
use iup::control::{Button, Text};
use iup::layout::{VBox, HBox, Fill};
use iup::image::{Image, ImageElement};
use iup::callback::{Action, CallbackReturn};
use iup::callback::button::{MouseButton, MouseButtonState, ButtonCb};

fn main() {
    iup::with_iup(|| {

        // We can create a image inline to be embedded in a control...
        let img_release = Image::with(pixels![
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,4,4,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,4,4,4,4,3,3,3,2,2],
            [1,1,3,3,3,3,3,4,4,4,4,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,4,4,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,1,3,3,3,3,3,3,3,3,3,3,3,3,2,2],
            [1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],
            [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]
        ]).set_colors([(0, 0, 0), (215,215,215), (40, 40, 40), (30, 50, 210), (240, 0, 0)]);

        // Creates a text box
        let mut text = Text::new()
                            .set_attrib("READONLY", "YES")
                            .set_attrib("EXPAND", "YES");

        // Creates a button with image
        let mut btn_image = Button::with_title("Button with image")
                                .set_attrib_handle("IMAGE", img_release)
                                .set_button_cb(move |(_, button, state, _, _, _)| {
                                    if button == MouseButton::Button1 {
                                        text.set_attrib("VALUE", format!("Odd button {}", match state {
                                            MouseButtonState::Pressed => "pressed",
                                            MouseButtonState::Released => "released",
                                        }));
                                    }
                                });

        // Creates a [useless] button
        let btn_big = Button::with_title("Big useless button")
                                .set_attrib("SIZE", "0.125x0.125");

        // Creates a button entitled Exit
        let btn_exit = Button::with_title("Exit")
                                .set_action(|_| CallbackReturn::Close);

        // Creates a button entitled 'Activate'
        let btn_on_off = Button::with_title("Activate")
                                .set_action(move |_| {
                                    if btn_image.attrib("ACTIVE").unwrap() == "YES" {
                                        btn_image.set_attrib("ACTIVE", "NO");
                                    } else {
                                        btn_image.set_attrib("ACTIVE", "YES");
                                    }
                                });

        // Creates a dialog window
        Dialog::new(
            VBox::new(elements![
                HBox::new(elements![
                    Fill::new(), btn_image, btn_on_off, btn_exit, Fill::new()
                ]),
                text, btn_big
            ])
        ).set_attrib("TITLE", "Button")
         .set_attrib("RESIZE", "NO")        // Turn of resize
         .set_attrib("MENUBOX", "NO")       // ... menu box
         .set_attrib("MAXBOX", "NO")        // ... maximize
         .set_attrib("MINBOX", "NO")        // and minimize
         .show()

    }).unwrap();
}
