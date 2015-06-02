//! An example on toggle and radio controls.
//!
//! This example contains:
//!   + One 3 state button that does nothing.
//!   + Four toggles that switch the color of the dialog window.
//!   + One button that activates or deactivates the coloring of the dialog window.
//!

#[macro_use]
extern crate iup;

use std::rc::Rc;
use std::cell::Cell;

use iup::{Element, Orientation};
use iup::dialog::Dialog;
use iup::layout::{Radio, VBox};
use iup::control::{Frame, Toggle, ToggleAction, Label};

const RED: (u8, u8, u8)   = (255, 0, 0);
const GREEN: (u8, u8, u8) = (0, 255, 0);
const BLUE: (u8, u8, u8)  = (0, 0, 255);

fn change_color(mut dialog: Dialog, state: bool, color: Option<(u8, u8, u8)>)
                                                                -> Option<(u8, u8, u8)> {
    if state {
        match color {
            Some(rgb) => dialog.set_attrib_rgb("BGCOLOR", rgb),
            None => dialog.clear_attrib("BGCOLOR"),
        };
    }
    dialog.attrib_rgb("BGCOLOR")
}

fn main() {
    iup::with_iup(|| {

        // Since we are going to share the state of this color in our callbacks we need to put it
        // in a reference counted cell, then clone a instance for each callback.
        let color = Rc::new(Cell::new(None));
        let (color_d, color_r) = (color.clone(), color.clone());
        let (color_g, color_b) = (color.clone(), color.clone());

        // We instantiate the dialog up here with no child so we can access it from the toggle
        // callbacks...
        let mut dialog = Dialog::new_empty();

        // Setup the useless toggle
        let toggle3s = Toggle::with_title("Useless Toggle")
                        .set_attrib("3STATE", "YES");

        // Setup the coloring toggles
        let toggle_d = Toggle::with_title("Default Color")
                        .set_action(move |(_, state)| color_d.set(change_color(dialog, state, None)));
        let toggle_r = Toggle::with_title("Red Color")
                        .set_action(move |(_, state)| color_r.set(change_color(dialog, state, Some(RED))));
        let toggle_g = Toggle::with_title("Green Color")
                        .set_action(move |(_, state)| color_g.set(change_color(dialog, state, Some(GREEN))));
        let toggle_b = Toggle::with_title("Blue Color")
                        .set_action(move |(_, state)| color_b.set(change_color(dialog, state, Some(BLUE))));

        // Setup the radio of mutually exclusive toggles
        let mut radio = Radio::new(
            Frame::new(
                VBox::new(elements![
                    toggle_d,
                    toggle_r,
                    toggle_g,
                    toggle_b,
                ])
            ).set_attrib("TITLE", "Colors")
        );

        // Setup the allow colors toggle
        let toggle = Toggle::with_title("Allow Colors")
                        .set_attrib("VALUE", "YES")
                        .set_action(move |(_, state)| {
                            if state {
                                radio.set_attrib("ACTIVE", "YES");
                                color.set(change_color(dialog, true, color.get()));
                            } else {
                                radio.set_attrib("ACTIVE", "NO");
                                change_color(dialog, true, None);
                            }
                        });

        // Add a layout to the dialog hierarchy
        dialog.append(
            VBox::new(elements![
                toggle3s,
                Label::new_separator(Orientation::Horizontal),
                toggle,
                radio,
            ])
        ).unwrap();

        // Setup the dialog and show it up
        dialog
         .set_attrib("TITLE", "IupToggle")
         .set_attrib("MARGIN", "5x5")
         .set_attrib("GAP", "5")
         .set_attrib("RESIZE", "NO")
         .show()

    }).unwrap();
}
