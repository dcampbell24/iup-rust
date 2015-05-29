// hello1.rs example written in LED.
#[macro_use]
extern crate iup;

use iup::{Element, Handle};
use iup::dialog::Dialog;
use iup::control::Button;
use iup::callback::{Action, CallbackReturn};
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

    	let mut dialog = Dialog::from_handle(Handle::from_name("dlg").unwrap()).unwrap();
		let mut button = Button::from_handle(Handle::from_name("btn").unwrap()).unwrap();
		button.set_action(|_| CallbackReturn::Close);

		dialog.show()

    }).unwrap();
}
