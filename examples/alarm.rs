//! Example based on http://sourceforge.net/p/iup/iup/HEAD/tree/trunk/iup/html/examples/Lua/alarm.lua

#[macro_use]
extern crate iup;

use iup::prelude::*;
use iup::dialog::AlarmButton;

use std::result::Result;

fn main () {
    iup::with_iup(|| {
        match iup::dialog::alarm("Alarm Example", "File not saved! Save it now?", "Yes".into(),
                                 Some("No".into()), Some("Cancel".into())) {
            AlarmButton::Button1 => iup::dialog::message("Save file", "File saved successfully - leaving program"),
            AlarmButton::Button2 => iup::dialog::message("Save file", "File not saved - leaving program anyway"),
            AlarmButton::Button3 => iup::dialog::message("Save file", "Operation canceled"),
        }

        // don't let the main loop run or we'll get frozen because we didn't initialise any dialog.
        let fixme: Result<(), ()> = Err(());    // TODO FIXME not able to infer enought type information
        fixme
    }).unwrap();
}
