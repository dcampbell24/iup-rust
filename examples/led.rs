// hello1.rs example written in LED.
#[macro_use]
extern crate iup;

use iup::prelude::*;
use iup::Handle;
use iup::control::Button;
use iup::led;

fn main () {
    iup::with_iup(|| {
        // See also led::load(path) to load from a file
        led::load_buffer(r######"
            # This is a LED comment.
            btn = button[EXPAND=YES, TIP="Exit button"]("Ok", 0)
            dlg = dialog[TITLE="Hello"]
            (
                vbox[GAP=10, MARGIN=10x10, ALIGNMENT=ACENTER]
                (
                    label("Hello, world!"),
                    btn
                )
            )
        "######).unwrap();

        let mut dialog = Dialog::from_handle(Handle::from_named("dlg").unwrap()).unwrap();
        let mut button = Button::from_handle(Handle::from_named("btn").unwrap()).unwrap();
        button.set_action(|_| CallbackReturn::Close);

        dialog.show()

    }).unwrap();
}
